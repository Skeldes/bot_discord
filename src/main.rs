use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
    utils::MessageBuilder
};

use std::env;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message){
        if msg.content == "!ping" {

            let response = MessageBuilder::new()
                .push("User ")
                .push(&msg.author)
                .push(" why are you pinging me ?")
                .build();

            if let Err(why) = msg.channel_id.say(&ctx.http, &response).await {
                println!("Error sending message {:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready : Ready){
        println!("{} is connected !", ready.user.name);
    }
}

#[tokio::main]
async fn main() {

    dotenv::dotenv().expect("Failed to load .env file");

    let token = env::var("BOT_TOKEN").expect("Expected a token in the environment");

    let mut client = Client::builder(&token).event_handler(Handler).await.expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error : {:?}", why);
    }
}