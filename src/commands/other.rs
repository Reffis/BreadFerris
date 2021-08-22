use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use breadferris::cmdlog;

#[command]
#[aliases("샍", "샌즈", "샍즈", "샌주", "샍주")]
async fn sans(_ctx: &Context, msg: &Message) -> CommandResult {
    cmdlog(&msg.author.id, &msg.content);
    Ok(())
}
