use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::time::Instant;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    let api_latency = {
        let instant = Instant::now();
        msg.channel_id.broadcast_typing(&ctx.http).await?;
        instant.elapsed().as_millis() as f64
    };

    msg.reply(ctx, format!("Pong! 🏓\nAPI Latency: {}ms", api_latency))
        .await?;

    Ok(())
}