//use super::image_lib::*;
use super::embed_colors::*;
use crate::commands::*;
use breadferris::{cmdlog, loadconfig};
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::interactions::message_component::ButtonStyle;
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::time::{Duration, Instant};

#[command]
#[aliases("핑")]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(
        ctx,
        format!("Pong! 🏓\nAPI Latency: {}ms", {
            let instant = Instant::now();
            msg.channel_id.broadcast_typing(&ctx.http).await?;
            instant.elapsed().as_millis() as f64
        }),
    )
    .await?;
    cmdlog(msg.author.id.to_string(), msg.content.clone());
    Ok(())
}

#[command]
#[aliases("도움말", "도움")]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    let mut m = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.components(|c| {
                c.create_action_row(|a| {
                    a.create_button(|b| {
                        b.label("Utility")
                            .style(ButtonStyle::Success)
                            .custom_id("HELP_UTIL")
                            .emoji(ReactionType::from('🎈'))
                    })
                    .create_button(|b| {
                        b.label("IMAGE")
                            .style(ButtonStyle::Danger)
                            .custom_id("HELP_IMAGE")
                            .emoji(ReactionType::from('🖼'))
                    })
                    .create_button(|b| {
                        b.label("Moderator")
                            .style(ButtonStyle::Secondary)
                            .custom_id("HELP_MODER")
                            .emoji(ReactionType::from('🛠'))
                    })
                })
                .create_action_row(|bbb| {
                    bbb.create_button(|b| {
                        b.label("FUN")
                            .style(ButtonStyle::Success)
                            .custom_id("HELP_FUN")
                            .emoji(ReactionType::from('🧊'))
                    })
                    .create_button(|b| {
                        b.label("OWNER")
                            .style(ButtonStyle::Secondary)
                            .custom_id("HELP_OWNER")
                            .emoji(ReactionType::from('🛡'))
                    })
                    .create_button(|b| {
                        b.label("Delete")
                            .style(ButtonStyle::Danger)
                            .custom_id("HELP_DEL")
                            .emoji(ReactionType::from('🗑'))
                    })
                    .create_button(|b| {
                        b.label("OpenSource")
                            .style(ButtonStyle::Link)
                            .url("https://github.com/Reffis/BreadFerris")
                    })
                })
            })
            .embed(|x| {
                x.title("Help")
                    .description(
                        r#"
아래의 버튼을 눌러 도움말을 확인하세요.

채널 주제가 `-페리스공지-` 라면, 해당 채널에 공지 메세지가 올라옵니다.

⚠ **명령어를 확인하셨으면, 반드시 `Delete` 버튼을 눌러주세요.**
"#,
                    )
                    .colour(random_color())
                    .footer(|f| {
                        f.text("OpenSource: https://github.com/Reffis/breadferris");
                        f.icon_url("https://avatars.githubusercontent.com/u/88228766?s=200&v=4")
                    })
            })
        })
        .await
        .unwrap();
    while let Some(interaction_data) = m
        .await_component_interaction(ctx)
        .author_id(msg.author.id)
        .channel_id(msg.channel_id)
        .message_id(m.id)
        .collect_limit(1)
        .timeout(std::time::Duration::from_secs(10))
        .await
    {
        let t = interaction_data.data.custom_id.as_str();
        if t == "HELP_UTIL" {
            m.edit(&ctx.http, |f| {
                f.embed(|x| {
                    x.title("🎈 - Utility")
                        .description(HELP_UTIL)
                        .colour(random_color())
                        .footer(|f| {
                            f.text("OpenSource: https://github.com/Reffis/breadferris");
                            f.icon_url("https://avatars.githubusercontent.com/u/88228766?s=200&v=4")
                        })
                })
            })
            .await?;
            interaction_data
                .create_interaction_response(ctx, |f| {
                    f.kind(InteractionResponseType::DeferredUpdateMessage)
                })
                .await
                .unwrap_or_default();
        } else if t == "HELP_IMAGE" {
            m.edit(&ctx.http, |f| {
                f.embed(|x| {
                    x.title("🖼️ - Image")
                        .description(HELP_IMAGE)
                        .colour(random_color())
                        .footer(|f| {
                            f.text("OpenSource: https://github.com/Reffis/breadferris");
                            f.icon_url("https://avatars.githubusercontent.com/u/88228766?s=200&v=4")
                        })
                })
            })
            .await?;
            interaction_data
                .create_interaction_response(ctx, |f| {
                    f.kind(InteractionResponseType::DeferredUpdateMessage)
                })
                .await
                .unwrap_or_default();
        } else if t == "HELP_MODER" {
            m.edit(&ctx.http, |f| {
                f.embed(|x| {
                    x.title("🛠️ - Moderator")
                        .description(HELP_MODER)
                        .colour(random_color())
                        .footer(|f| {
                            f.text("OpenSource: https://github.com/Reffis/breadferris");
                            f.icon_url("https://avatars.githubusercontent.com/u/88228766?s=200&v=4")
                        })
                })
            })
            .await?;
            interaction_data
                .create_interaction_response(ctx, |f| {
                    f.kind(InteractionResponseType::DeferredUpdateMessage)
                })
                .await
                .unwrap_or_default();
        } else if t == "HELP_OWNER" {
            m.edit(&ctx.http, |f| {
                f.embed(|x| {
                    x.title("🛡️ - Owner")
                        .description(HELP_OWNER)
                        .colour(random_color())
                        .footer(|f| {
                            f.text("OpenSource: https://github.com/Reffis/breadferris");
                            f.icon_url("https://avatars.githubusercontent.com/u/88228766?s=200&v=4")
                        })
                })
            })
            .await?;
            interaction_data
                .create_interaction_response(ctx, |f| {
                    f.kind(InteractionResponseType::DeferredUpdateMessage)
                })
                .await
                .unwrap_or_default();
        } else if t == "HELP_DEL" {
            m.delete(&ctx.http).await?;
            break;
        } else if t == "HELP_FUN" {
            m.edit(&ctx.http, |f| {
                f.embed(|x| {
                    x.title("🧊 - FUN")
                        .description(HELP_FUN)
                        .colour(random_color())
                        .footer(|f| {
                            f.text("OpenSource: https://github.com/Reffis/breadferris");
                            f.icon_url("https://avatars.githubusercontent.com/u/88228766?s=200&v=4")
                        })
                })
            })
            .await?;
            interaction_data
                .create_interaction_response(ctx, |f| {
                    f.kind(InteractionResponseType::DeferredUpdateMessage)
                })
                .await
                .unwrap_or_default();
        }
    }
    cmdlog(msg.author.id.to_string(), msg.content.clone());
    Ok(())
}

#[command]
#[aliases("개발자", "제작자", "developer")]
async fn dev(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.colour(YELLOW)
                    .title("Help")
                    .description("`! Bread Cat#0002` (760688241447141395)")
                    .url("https://github.com/fn79")
                    .thumbnail("https://cdn.discordapp.com/avatars/760688241447141395/a_3a5a1997eb58c5360d9d0395e32f3417.gif?size=1024")
                    .field(
                        "개발환경",
                        r#"
> **Rust Version:** cargo 1.54.0 (2021-06-22)

> **Editor:** IntelliJ ~~(or VSCode)~~

> **OS:** Windows 10 - 20H2 (OS Build 19042.1110)

> **Terminal:** Powershell (or CMD)

> **Rust Serenity**
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
                        e.colour(GREEN)
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
#[aliases("실행", "eval")]
async fn run(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let message = msg.reply(ctx, "잠시만 기다려주세요 . . .\n\n`기다려도 응답이 없나요?`\n`출력되는 글자수가 너무 많거나, 서버에 문제가 있을수도 있어요.`").await?;
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

    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.colour(WHITE)
                    .title("Rust Playground")
                    .url("https://play.rust-lang.org/")
                    .field("stderr", format!("```rs\n{}\n```", json["stderr"]), false)
                    .field("stdout", format!("```rs\n{}\n```", json["stdout"]), false)
            })
        })
        .await?;

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

    let user_nick = user
        .nick_in(&ctx.http, msg.guild_id.unwrap_or_default())
        .await
        .unwrap_or_else(|| "None".to_string());
    let full_name = format!("{}#{}", user.name, user.discriminator);
    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.colour(BLUE)
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

#[command]
#[aliases("서버이모지", "이모지")]
async fn server_emoji(ctx: &Context, msg: &Message) -> CommandResult {
    let mut emoji_list = String::from("```\n");
    let emoji = msg.guild_id.unwrap_or_default().emojis(ctx).await?;
    emoji.iter().enumerate().for_each(|i| {
        if i.1.animated {
            emoji_list.push_str(format!("<a:{}:{}>\n", i.1.name, i.1.id).as_str());
        } else {
            emoji_list.push_str(format!("<:{}:{}>\n", i.1.name, i.1.id).as_str());
        }
    });
    emoji_list.push_str("```");
    if let Err(_) = msg.reply(ctx, emoji_list).await {
        msg.reply(
            ctx,
            "으에에? 아무래도 이모지가 너무 많아서 출력을 못하는거 같아요..",
        )
        .await?;
    }
    Ok(())
}

#[command]
#[aliases("초대", "봇초대")]
async fn invite(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
           e.colour(WHITE)
               .title("봇 초대 & 오픈소스")
               .description(r#"
**봇 초대 (관리자) [(클릭)](https://discord.com/api/oauth2/authorize?client_id=785702034388287518&permissions=8&scope=bot)**
**봇 초대 (권한 없음) [(클릭)](https://discord.com/api/oauth2/authorize?client_id=785702034388287518&permissions=0&scope=bot)**

**오픈소스 [(클릭)](https://github.com/Reffis/BreadFerris)**
               "#)
        })
    }).await?;
    Ok(())
}
