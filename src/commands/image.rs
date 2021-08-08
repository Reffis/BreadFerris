use json;
use reqwest;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use json::JsonValue;

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
                e.colour(0xBBFFFF)
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
                e.colour(0xBBFFFF)
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
    Ok(())
}

#[command]
#[aliases("고양이", "야옹이", "애옹")]
async fn cat(ctx: &Context, msg: &Message) -> CommandResult {
    let r = reqwest::get("https://api.thecatapi.com/v1/images/search")
        .await?
        .text()
        .await?;
    let image = &json::parse(r.as_str())?[0]["url"];
    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.colour(0xBBFFFF)
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
                    e.colour(0xBBFFFF)
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
    Ok(())
}