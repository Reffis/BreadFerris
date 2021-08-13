use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::embed_colors::WHITE;
use breadferris::cmdlog;

#[command]
#[aliases("샍", "샌즈", "샍즈", "샌주", "샍주")]
async fn sans(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "헋 샍주 아시는구나!").await?;
    cmdlog(msg.author.id.to_string(), msg.content.clone());
    Ok(())
}
