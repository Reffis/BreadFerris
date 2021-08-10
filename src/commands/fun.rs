use breadferris::cmdlog;
use rand::prelude::SliceRandom;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[aliases("랜덤", "골라", "random")]
async fn choice(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let r = args.rest().split(", ").collect::<Vec<_>>();
    cmdlog(msg.author.id.to_string(), msg.content.clone());
    msg.reply(
        ctx,
        format!(
            "`{}`",
            r.choose(&mut rand::thread_rng()).unwrap_or_else(|| &"")
        ),
    )
    .await?;
    Ok(())
}

#[command]
#[aliases("니트로")]
async fn nitro(ctx: &Context, msg: &Message) -> CommandResult {
    let mut v = String::new();
    (0..16).enumerate().for_each(|_| {
        v.push(
            *({
                let mut _v = ('a'..='z').collect::<Vec<_>>();
                _v.append(&mut ('A'..='Z').collect::<Vec<_>>());
                _v
            }
            .choose(&mut rand::thread_rng()))
            .unwrap_or_else(|| &' '),
        );
    });
    msg.reply(ctx, format!("`discord.gift/{}`", v)).await?;

    cmdlog(msg.author.id.to_string(), msg.content.clone());
    Ok(())
}
