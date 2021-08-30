use super::embed_colors::*;
use crate::commands::NEKOTYPE;
use bbanglog::info;
use json;
use json::JsonValue;
use reqwest;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[aliases("여우")]
async fn fox(ctx: &Context, msg: &Message) -> CommandResult {
    let r = json::parse(
        reqwest::get("https://randomfox.ca/floof/")
            .await?
            .text()
            .await?
            .as_str(),
    )?["image"]
        .to_string();
    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.colour(INDIGO)
                    .title("Fox")
                    .url("https://randomfox.ca/floof/")
                    .image(r.as_str())
                    .footer(|f| {
                        f.text(format!("{}", msg.author.name));
                        f.icon_url(msg.author.avatar_url().unwrap_or_default())
                    })
            })
        })
        .await?;
    info!("Command: [{}] {}", &msg.author.id, &msg.content);
    Ok(())
}

#[command]
#[aliases("시바견")]
async fn shiba(ctx: &Context, msg: &Message) -> CommandResult {
    let r = reqwest::get("http://shibe.online/api/shibes?urls=true&httpsUrls=true")
        .await?
        .text()
        .await?;
    let image = &json::parse(r.as_str()).unwrap()[0];
    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.colour(RED)
                    .title("Shiba")
                    .url("http://shibe.online/api/shibes?urls=true&httpsUrls=true")
                    .image(image)
                    .footer(|f| {
                        f.text(format!("{}", msg.author.name));
                        f.icon_url(msg.author.avatar_url().unwrap_or_default())
                    })
            })
        })
        .await?;
    info!("Command: [{}] {}", &msg.author.id, &msg.content);
    Ok(())
}

#[command]
#[aliases("고양이", "야옹이", "애옹", "kitty")]
async fn cat(ctx: &Context, msg: &Message) -> CommandResult {
    let r = reqwest::get("https://api.thecatapi.com/v1/images/search")
        .await?
        .text()
        .await?;
    let image = &json::parse(r.as_str())?[0]["url"];
    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.colour(ORANGE)
                    .title("Cat")
                    .url("https://api.thecatapi.com/v1/images/search")
                    .image(image)
                    .footer(|f| {
                        f.text(format!("{}", msg.author.name));
                        f.icon_url(msg.author.avatar_url().unwrap_or_default())
                    })
            })
        })
        .await?;
    info!("Command: [{}] {}", &msg.author.id, &msg.content);
    Ok(())
}

#[command]
#[aliases("밈")]
async fn meme(ctx: &Context, msg: &Message) -> CommandResult {
    let r = reqwest::get("https://meme-api.herokuapp.com/gimme")
        .await?
        .text()
        .await?;
    let data = &json::parse(r.as_str())?;
    let title = &data["title"];
    let url = &data["url"];
    let postlink = &data["postLink"];
    if &data["nsfw"] == &JsonValue::Boolean(true) {
        // TODO
    } else {
        msg.channel_id
            .send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.colour(GREEN)
                        .title(title)
                        .url(postlink)
                        .image(url)
                        .footer(|f| {
                            f.text(format!("{}", msg.author.name));
                            f.icon_url(msg.author.avatar_url().unwrap_or_default())
                        })
                })
            })
            .await?;
    }
    info!("Command: [{}] {}", &msg.author.id, &msg.content);
    Ok(())
}

#[command]
#[aliases("네코")]
async fn neko(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if let Some(c) = msg.channel(&ctx.cache).await {
        if c.is_nsfw() {
            if args.rest().is_empty() {
                msg.reply(
                    ctx,
                    "`ferris neko help` 를 입력하여, 도움말을 확인해주세요.",
                )
                .await?;
            } else if args.rest() == "help" {
                msg.reply(ctx, NEKOTYPE).await?;
            } else {
                let r = reqwest::get(format!("https://nekos.life/api/v2/img/{}", args.rest()))
                    .await?
                    .text()
                    .await?;
                let d = &json::parse(r.as_str())?;
                if d["msg"] == "404" {
                    msg.reply(ctx, "알수없는 이름입니다. `ferris neko help`")
                        .await?;
                } else {
                    let url = &d["url"];
                    msg.channel_id
                        .send_message(&ctx.http, |m| {
                            m.embed(|e| {
                                e.colour(random_color())
                                    .title(args.rest())
                                    .url(url)
                                    .image(url)
                                    .footer(|f| {
                                        f.text(format!("{}", msg.author.name));
                                        f.icon_url(msg.author.avatar_url().unwrap_or_default())
                                    })
                            })
                        })
                        .await?;
                }
            }
        } else {
            msg.reply(ctx, "해당채널에서는 사용할 수 없는 명령어입니다.\n사용을 원한다면, `nsfw` 채널로 설정해주세요.").await?;
        }
    }
    info!("Command: [{}] {}", &msg.author.id, &msg.content);
    Ok(())
}

#[command]
#[aliases("코기", "웰시코기", "애옹")]
async fn corgi(ctx: &Context, msg: &Message) -> CommandResult {
    let r = reqwest::get("https://dog.ceo/api/breed/pembroke/images/random")
        .await?
        .text()
        .await?;
    let image = &json::parse(r.as_str())?["message"];
    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.colour(YELLOW)
                    .title("Corgi")
                    .url(image)
                    .image(image)
                    .footer(|f| {
                        f.text(format!("{}", msg.author.name));
                        f.icon_url(msg.author.avatar_url().unwrap_or_default())
                    })
            })
        })
        .await?;
    info!("Command: [{}] {}", &msg.author.id, &msg.content);
    Ok(())
}
