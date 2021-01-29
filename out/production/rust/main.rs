use std::env;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

const COMMAND: &str = "!yazitura";

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == COMMAND {
            let possibilities = vec!["YAZI", "TURA"];
            let result: Vec<_> = possibilities
                .choose_multiple(&mut rand::thread_rng(), 1)
                .collect();
            if let Err(why) = msg.channel_id.say(&ctx.http, "<#804687062552412181> icin sonuc:" + result + ":coin:").await {
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
    let token = env::var("ODA0Njg0MzQxNjQwNTYwNjUw.YBP6sQ._-DG5kLBXJMdi4ABDw2DH-6NRrc")
        .expect("Expected a token in the environment");

    let mut client = Client::new(&token)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}