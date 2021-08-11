use serenity::framework::standard::Args;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use breadferris::LogType::Info;
use breadferris::{cmdlog, log};

use crate::libs::eval_lib;
use crate::ShardManagerContainer;

#[command]
#[owners_only]
async fn eval(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let r = args
        .rest()
        .split("\n")
        .filter(|x| match x {
            &"```" | &"```rs" => false,
            _ => true,
        })
        .map(|x| x.to_string() + "\n")
        .collect::<String>();
    msg.reply(
        ctx,
        format!(
            "```rs\n{}\n```",
            result = match eval_lib::eval(r.as_str(), true) {
                Ok(e) => e,
                Err(e) => format!("Error: {}", e),
            }
            .as_str()
        ),
    )
    .await?;
    cmdlog(msg.author.id.to_string(), msg.content.clone());
    Ok(())
}

#[command]
#[aliases("종료", "shutdown", "exit")]
#[owners_only]
async fn quit(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;

    if let Some(manager) = data.get::<ShardManagerContainer>() {
        manager.lock().await.shutdown_all().await;
    } else {
        msg.reply(ctx, "Err").await?;
        return Ok(());
    }

    log(Info, format!("Shutdown. . ."));
    cmdlog(msg.author.id.to_string(), msg.content.clone());
    Ok(())
}

#[command]
#[aliases("상태", "상메", "상태메세지", "게임상태")]
#[owners_only]
async fn status(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    ctx.set_activity(Activity::playing(args.rest())).await;
    msg.reply(ctx, format!("```diff\n+ Text: {}\n```", args.rest()))
        .await?;
    cmdlog(msg.author.id.to_string(), msg.content.clone());
    Ok(())
}

#[command]
#[aliases("nickname", "닉네임", "닉")]
#[owners_only]
async fn nick(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    msg.guild_id
        .unwrap()
        .edit_nickname(&ctx.http, Some(args.rest()))
        .await?;
    Ok(())
}

#[command]
#[aliases("messsage_delete", "메세지삭제")]
#[owners_only]
async fn msg_del(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let msg = ctx
        .http
        .get_message(msg.channel_id.0, args.single::<u64>()?)
        .await?;
    msg.delete(ctx).await?;
    Ok(())
}
