use bbanglog::info;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[aliases("샍", "샌즈", "샍즈", "샌주", "샍주")]
async fn sans(_ctx: &Context, msg: &Message) -> CommandResult {
    info!("Command: [{}] {}", &msg.author.id, &msg.content);
    Ok(())
}
