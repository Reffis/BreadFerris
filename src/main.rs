mod commands;
mod libs;
use std::{collections::HashSet, sync::Arc};

use breadferris::{loadconfig, log, LogType::*};
use commands::image::*;
use commands::owner::*;
use commands::util::*;
use serenity::model::gateway::Activity;
use serenity::{
    async_trait,
    client::bridge::gateway::ShardManager,
    framework::{standard::macros::group, StandardFramework},
    http::Http,
    model::{event::ResumedEvent, gateway::Ready},
    prelude::*,
};
use std::process::exit;

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        log(Info, format!("Connected as {}", ready.user.name));
        ctx.set_activity(Activity::playing(format!(
            "ferris help / {} Servers",
            ready.guilds.len()
        )))
        .await;
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        log(Info, format!("Resumed"));
    }
}

#[group]
#[commands(eval, quit, status)]
struct General;

#[group]
#[commands(ping, help, support, dev, run)]
struct Utility;

#[group]
#[commands(shiba, fox, cat, meme)]
struct Image;

#[tokio::main]
async fn main() {
    let token = loadconfig("token".to_string());

    let http = Http::new_with_token(&token);

    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        }
        Err(why) => {
            log(
                Error,
                format!("Could not access application info: {:?}", why),
            );
            exit(1);
        }
    };

    // Create the framework
    let framework = StandardFramework::new()
        .configure(|c| {
            c.owners(owners).prefixes([
                loadconfig("prefix".to_string()),
                loadconfig("prefix2".to_string()),
            ])
        })
        .group(&GENERAL_GROUP)
        .group(&UTILITY_GROUP)
        .group(&IMAGE_GROUP);

    let mut client = Client::builder(&token)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Could not register ctrl+c handler");
        shard_manager.lock().await.shutdown_all().await;
    });

    if let Err(e) = client.start().await {
        println!("실패: {:?}", e);
    }
}
