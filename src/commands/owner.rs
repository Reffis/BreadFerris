use serenity::framework::standard::Args;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use crate::ShardManagerContainer;

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

    Ok(())
}
