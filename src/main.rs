use nominatim::NominatimError;
use std::env;

use rand::seq::IteratorRandom;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct Bar {
    name: &'static str,
    location: String,
}

impl Bar {
    pub async fn find_by_name(name: &'static str) -> Result<Self, NominatimError> {
        let location = nominatim::NominatimClient {
            identification: nominatim::IdentificationMethod::UserAgent("ostdb".to_owned()),
        }
        .search(format!("{} grenoble", name))
        .await
        .map(|x| format!("https://www.openstreetmap.org/node/{}", x.osm_id))?;

        Ok(Self { name, location })
    }
}

struct Handler(Vec<Bar>);

const BARS: &'static str = include_str!("../bars.txt");

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let text = msg.content.to_lowercase();
        if text.starts_with("bar") && text.ends_with("?") {
            let bar = {
                let mut rng = rand::thread_rng();
                self.0.iter().choose(&mut rng).unwrap()
            };

            if let Err(why) = msg
                .channel_id
                .say(&ctx.http, format!("{} {}", bar.name, bar.location))
                .await
            {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    println!("building list of bars");
    let mut bars = Vec::new();
    for bar in BARS.lines() {
        match Bar::find_by_name(bar).await {
            Ok(bar) => bars.push(bar),
            Err(err) => eprintln!("warn: bar {bar:?} wasn't found on Nominatim: {err}"),
        }
    }
    assert!(bars.len() > 0, "the list of bars cannot be empty");
    println!("done, we have {} bars in stock", bars.len());

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let mut client = Client::builder(&token)
        .event_handler(Handler(bars))
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
