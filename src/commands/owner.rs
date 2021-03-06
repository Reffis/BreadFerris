use bbanglog::info;
use serenity::framework::standard::Args;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use super::embed_colors::*;

use crate::ShardManagerContainer;

#[command]
#[aliases("종료", "shutdown", "exit")]
#[owners_only]
async fn quit(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;

    if let Some(manager) = data.get::<ShardManagerContainer>() {
        msg.reply(ctx, "미국으로 가는중. . .").await?;
        manager.lock().await.shutdown_all().await;
    } else {
        msg.reply(ctx, "Err").await?;
        return Ok(());
    }

    info!("Shutdown. . .");
    info!("Command: [{}] {}", &msg.author.id, &msg.content);
    Ok(())
}

#[command]
#[aliases("상태", "상메", "상태메세지", "게임상태")]
#[owners_only]
async fn status(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    ctx.set_activity(Activity::playing(args.rest())).await;
    msg.reply(ctx, format!("```diff\n+ Text: {}\n```", args.rest()))
        .await?;
    info!("Command: [{}] {}", &msg.author.id, &msg.content);
    Ok(())
}

#[command]
#[aliases("상태업데이트")]
#[owners_only]
async fn status_update(ctx: &Context, msg: &Message) -> CommandResult {
    ctx.set_activity(Activity::playing(format!(
        "ferris help / {} Servers",
        ctx.cache.guilds().await.len()
    )))
    .await;
    msg.reply(ctx, "상태를 업데이트 하였습니다.").await?;
    info!("Command: [{}] {}", &msg.author.id, &msg.content);
    Ok(())
}

#[command]
#[aliases("nickname", "닉네임", "닉")]
#[owners_only]
async fn nick(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    msg.guild_id
        .unwrap_or_default()
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

#[command]
#[aliases("공지")]
#[owners_only]
async fn announcements(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    for guild in ctx.cache.guilds().await {
        for channel in guild.channels(&ctx.http).await? {
            if channel
                .1
                .topic
                .unwrap_or_default()
                .contains("-FerrisAnnouncements")
            {
                channel
                    .0
                    .send_message(&ctx.http, |m| {
                        m.embed(|e| {
                            e.colour(WHITE)
                                .title("BreadFerris - 공지")
                                .description(args.rest())
                                .footer(|f| {
                                    f.icon_url(msg.author.avatar_url().unwrap_or_default())
                                        .text(format!(
                                            "{}#{}",
                                            msg.author.name, msg.author.discriminator
                                        ))
                                })
                        })
                    })
                    .await?;
                msg.reply(
                    ctx,
                    format!(
                        "Channel: {} ({}), Guild: {} ({})",
                        channel.0.name(&ctx.cache).await.unwrap_or_default(),
                        channel.0 .0,
                        guild.name(&ctx.cache).await.unwrap_or_default(),
                        guild.0
                    ),
                )
                .await?;
            }
        }
    }
    info!("Command: [{}] {}", &msg.author.id, &msg.content);
    Ok(())
}
