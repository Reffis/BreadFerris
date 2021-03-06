pub extern crate bbanglog;

mod commands;
mod event_handler;

use bbanglog::*;
use breadferris::loadconfig;
use commands::fun::*;
use commands::image::*;
use commands::moderator::*;
use commands::other::*;
use commands::owner::*;
use commands::util::*;
use serenity::framework::standard::macros::hook;
use serenity::framework::standard::DispatchError;
use serenity::model::channel::Message;
use std::{collections::HashSet, sync::Arc};

use serenity::{
    client::bridge::gateway::ShardManager,
    framework::{standard::macros::group, StandardFramework},
    http::Http,
    prelude::*,
};
use std::process::exit;

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

#[group]
#[commands(quit, status, nick, msg_del, announcements, status_update)]
struct Owner;

#[group]
#[commands(
    ping,
    help,
    support,
    dev,
    run,
    opensource,
    uinfo,
    server_emoji,
    invite,
    go
)]
struct Utility;

#[group]
#[commands(shiba, fox, cat, meme, neko, corgi)]
struct Image;

#[group]
#[commands(sans)]
struct Other;

#[group]
#[commands(ban, kick, unban)]
struct Moderator;

#[group]
#[commands(
    choice,
    nitro,
    say,
    avatar,
    hangang,
    bbangcat,
    gunghab,
    owo,
    spoiler,
    nevergonnagiveyouup,
    ip
)]
struct Fun;

#[tokio::main]
async fn main() {
    let token = loadconfig().token.unwrap();

    let (owners, _bot_id) = match Http::new_with_token(&token)
        .get_current_application_info()
        .await
    {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        }
        Err(why) => {
            error!("Could not access application info: {:?}", why);
            exit(1);
        }
    };

    // Create the framework
    let framework = StandardFramework::new()
        .configure(|c| c.owners(owners).prefixes(loadconfig().prefix.unwrap()))
        .group(&OWNER_GROUP)
        .group(&UTILITY_GROUP)
        .group(&OTHER_GROUP)
        .group(&IMAGE_GROUP)
        .group(&MODERATOR_GROUP)
        .group(&FUN_GROUP)
        .on_dispatch_error(dispatch_error);

    let mut client = Client::builder(&token)
        .framework(framework)
        .event_handler(event_handler::Handler)
        .application_id(loadconfig().application_id.unwrap())
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

    client.start().await.unwrap();
}

#[hook]
async fn dispatch_error(_: &Context, msg: &Message, error: DispatchError) {
    error!(
        "Message: [{}], Author: [{}]: [{:?}]",
        msg.id, msg.author.id, error
    );
}
