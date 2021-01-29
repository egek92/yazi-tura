use std::env;
use rand::seq::SliceRandom;


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
            if let Err(why) = msg.channel_id.say(&ctx.http, get_random()).await {
                println!("Hata: {:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} baglandi!", ready.user.name);

    }
}

fn get_random() -> String {
    return vec!["YAZI","TURA"].choose(&mut rand::thread_rng()).unwrap().to_string();
}


#[tokio::main]
async fn main() {

    let token = env::var("")
        .expect("token gerekli");

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("client olusturulamadi");

    if let Err(why) = client.start().await {
        println!("client hatasi: {:?}", why);
    }
}