use crate::ShardManagerContainer;
use breadferris::log;
use breadferris::LogType::Info;
use serenity::framework::standard::Args;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[aliases("실행", "코드실행")]
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
            result = match super::eval_lib::eval(r.as_str(), true) {
                Ok(e) => e,
                Err(e) => format!("Error: {}", e),
            }
            .as_str()
        ),
    )
    .await?;

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

    Ok(())
}

#[command]
#[aliases("상태", "상메", "상태메세지", "게임상태")]
#[owners_only]
async fn status(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    ctx.set_activity(Activity::playing(args.rest())).await;
    msg.reply(ctx, format!("```diff\n+ Text: {}\n```", args.rest()))
        .await?;
    Ok(())
}
