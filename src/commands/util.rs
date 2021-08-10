//use super::image_lib::*;
use crate::commands::{HELP_FUN, HELP_IMAGE, HELP_MODER, HELP_OWNER, HELP_UTIL};
use breadferris::{cmdlog, loadconfig};
use json::JsonValue;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::time::Instant;

#[command]
#[aliases("핑")]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    let api_latency = {
        let instant = Instant::now();
        msg.channel_id.broadcast_typing(&ctx.http).await?;
        instant.elapsed().as_millis() as f64
    };

    msg.reply(ctx, format!("Pong! 🏓\nAPI Latency: {}ms", api_latency))
        .await?;
    cmdlog(msg.author.id.to_string(), msg.content.clone());
    Ok(())
}

#[command]
#[aliases("도움", "도움말")]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.colour(0xCC33CC)
                    .title("Help")
                    .description("prefix: `ferris`")
                    .field("Utility", HELP_UTIL, true)
                    .field("Image", HELP_IMAGE, true)
                    .field("Moderator", HELP_MODER, true)
                    .field("Owner", HELP_OWNER, true)
                    .field("Fun", HELP_FUN, true)
                    .footer(|f| {
                        f.text("OpenSource: https://github.com/Reffis/breadferris");
                        f.icon_url("https://avatars.githubusercontent.com/u/88228766?s=60&v=4")
                    })
            })
        })
        .await?;
    cmdlog(msg.author.id.to_string(), msg.content.clone());
    Ok(())
}

#[command]
#[aliases("개발자", "제작자", "developer")]
async fn dev(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.colour(0xCC33CC)
                    .title("Help")
                    .description("`! Bread Cat#0002` (760688241447141395)")
                    .url("https://github.com/fn79")
                    .thumbnail("https://cdn.discordapp.com/avatars/760688241447141395/a_3a5a1997eb58c5360d9d0395e32f3417.gif?size=1024")
                    .field(
                        "개발환경",
                        r#"
> cargo 1.54.0 (2021-06-22)

> IntelliJ (or VSCode)

> Windows 10 - 20H2 (OS Build 19042.1110)

> Powershell (or CMD)
                    "#,
                        true,
                    )
                    .footer(|f| {
                        f.text("OpenSource: https://github.com/Reffis/breadferris");
                        f.icon_url("https://avatars.githubusercontent.com/u/88228766?s=60&v=4")
                    })
            })
        })
        .await?;
    cmdlog(msg.author.id.to_string(), msg.content.clone());
    Ok(())
}

#[command]
#[aliases("문의", "지원")]
async fn support(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    match args.rest() {
        "" | " " => {
            msg.reply(ctx, "**문의 내용**을 입력해주세요.").await?;
        }
        _ => {
            let channel = ctx
                .http
                .get_channel(loadconfig("support_channel".to_string()).parse::<u64>()?)
                .await?;
            channel
                .id()
                .send_message(&ctx.http, |m| {
                    m.content(format!(
                        "**문의 - {} ({})**\n\n```\n{}\n```",
                        msg.author.name,
                        msg.author.id,
                        args.rest()
                    ))
                })
                .await?;
            msg.channel_id
                .send_message(&ctx.http, |m| {
                    m.embed(|e| {
                        e.colour(0x0000CD)
                            .title("문의가 전송되었습니다.")
                            .description(format!("내용:\n```\n{}\n```", args.rest()))
                            .footer(|f| {
                                f.text(format!(
                                    "{} - 장난식으로 문의하면 가만히 안둡니다! 흐헤헤",
                                    msg.author.name
                                ));
                                f.icon_url(msg.author.avatar_url().unwrap_or_default())
                            })
                    })
                })
                .await?;
        }
    }
    cmdlog(msg.author.id.to_string(), msg.content.clone());
    Ok(())
}

#[command]
#[aliases("실행")]
async fn run(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let message = msg.reply(ctx, "잠시만 기다려주세요 . . .").await?;
    let r = args
        .rest()
        .split("\n")
        .filter(|x| match x {
            &"```" | &"```rs" | &"`" => false,
            _ => true,
        })
        .map(|x| x.to_string() + " ")
        .collect::<String>()
        .replace("\"", "\\\"");
    let a = reqwest::Client::new();
    let format = format!(
        "
        {}\"channel\":\"stable\",
        \"mode\":\"debug\",
        \"edition\":\"2018\",
        \"crateType\":\"bin\",
        \"tests\":false,
        \"code\":\"{}\",
        \"backtrace\":false{}",
        "{\n", r, "\n}"
    );
    let res = a
        .post("https://play.rust-lang.org/execute")
        .header("content-type", "application/json")
        .body(format.clone())
        .send()
        .await?;
    let json = &json::parse(res.text().await?.as_str())?;
    if json["success"] == JsonValue::Boolean(true) {
        msg.reply(ctx, format!("```rs\n{}\n```", json["stdout"]))
            .await?;
    } else {
        msg.reply(ctx, format!("```rs\n{}\n```", json["stderr"]))
            .await?;
    }
    message.delete(ctx).await?;

    cmdlog(msg.author.id.to_string(), msg.content.clone());

    Ok(())
}

#[command]
#[aliases("오픈소스")]
async fn opensource(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(
        ctx,
        "**https://github.com/Reffis/BreadFerris**\n\n**Pull requests**는 언제나 환영입니다.",
    )
    .await?;
    Ok(())
}

#[command]
#[aliases("userinfo", "유저정보")]
async fn info(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let user = ctx
        .http
        .get_user(
            args.single::<String>()?
                .replace("<", "")
                .replace(">", "")
                .replace("@", "")
                .replace("!", "")
                .parse::<u64>()?,
        )
        .await?;

    let user_nick = user.nick_in(&ctx.http, msg.guild_id.unwrap_or_default()).await
        .unwrap_or_else(|| { "None".to_string() });
    let full_name = format!("{}#{}", user.name, user.discriminator);
    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.colour(0x00ffc8)
                    .title(format!("{} 님의 정보입니다.", full_name))
                    .footer(|f| {
                        f.text(msg.author.id)
                            .icon_url(user.avatar_url().unwrap_or_default())
                    })
                    .field(
                        "기본 정보",
                        format!(
                            r#"
**계정 이름: {}** ({})
계정 id: {}
**계정 생성일: {}**
봇 여부: {}
                        "#,
                            full_name,
                            user_nick,
                            user.id,
                            user.created_at(),
                            user.bot
                        ),
                        false,
                    )
                    .thumbnail(user.avatar_url().unwrap_or_default())
            })
        })
        .await?;

    cmdlog(msg.author.id.to_string(), msg.content.clone());
    Ok(())
}
