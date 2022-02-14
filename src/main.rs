mod parser;

use std::env;

use crate::parser::BarsFile;
use rand::seq::IteratorRandom;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

/// A bar ready to be shown by the Discord bot
struct BakedBar {
    name: String,
    osm_url: String,
}

struct Handler(Vec<BakedBar>);

const BARS: &'static str = include_str!("../bars.toml");

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
                .say(&ctx.http, format!("{} {}", bar.name, bar.osm_url))
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

    let bars_file = toml::from_str::<BarsFile>(BARS).unwrap();
    println!("we have {} bars in stock", bars_file.bars.len());

    let bars = bars_file
        .bars
        .into_iter()
        .map(|bar| BakedBar {
            name: bar.name,
            osm_url: bar.osm.to_string(),
        })
        .collect::<Vec<_>>();

    let mut client = Client::builder(&token)
        .event_handler(Handler(bars))
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
