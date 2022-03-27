mod commands;

use commands::{ping::*, math::*, owners::*, help::*};

use serenity::{
    async_trait,
    client::bridge::gateway::{
        GatewayIntents,
        ShardId,
        ShardManager,
    },
    framework::{
        standard::{
            macros::group,

        },
        StandardFramework
    },
    http::Http,
    model::{
        event::ResumedEvent,
        gateway::Ready
    },
    prelude::*,
};

use std::{
    collections::{
        HashMap,
        HashSet,
    },
    env,
    sync::Arc
};

use tracing::{error, info};


struct CommandCounter;

impl TypeMapKey for CommandCounter{
    type Value = HashMap<String, u64>;
}

#[group]
#[description = "A general group of commands"]
#[commands(ping, mult, statut)]
struct General; //Structure utilisé pour les commandes 


struct Handler; 

#[async_trait]
impl EventHandler for Handler {
    
    async fn ready(&self, _: Context, ready: Ready){
        info!("{} is connected !", ready.user.name);
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("resume");
    }
}


struct ShareManagerContainer;

impl TypeMapKey for ShareManagerContainer{
    type Value = Arc<Mutex<ShardManager>>;
}

#[tokio::main]
async fn main() {

    dotenv::dotenv().expect("Failed to load .env file");

    let token = env::var("BOT_TOKEN").expect("Expected a token in the environment");

    let http = Http::new_with_token(&token);

    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) =>{
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        },
        Err(why) => panic!("COuld not acces application info : {:?}", why),
    };

    let framework = StandardFramework::new()
        .configure(|c| c.owners(owners).prefix("!"))
        .help(&MY_HELP)    
        .group(&GENERAL_GROUP);

    let mut client = Client::builder(&token)
    .event_handler(Handler)
    .framework(framework)
    .await
    .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error : {:?}", why);
    }
}