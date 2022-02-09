use std::env;

use rand::seq::IteratorRandom;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct Handler;

const BARS: &'static str = include_str!("../bars.txt");

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let text = msg.content.to_lowercase();
        if text.starts_with("bar") && text.ends_with("?") {
            let bar = {
                let mut rng = rand::thread_rng();
                BARS.split("\n").choose(&mut rng).unwrap_or_default()
            };
            let loc = nominatim::NominatimClient {
                identification: nominatim::IdentificationMethod::UserAgent("ostdb".to_owned()),
            }
            .search(format!("{} grenoble", bar))
            .await
            .map(|x| format!("https://www.openstreetmap.org/node/{}", x.osm_id))
            .unwrap_or("(introuvable)".to_owned());
            if let Err(why) = msg
                .channel_id
                .say(&ctx.http, format!("{} {}", bar, loc))
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
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
