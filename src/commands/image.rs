use json;
use reqwest;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[aliases("여우")]
async fn fox(ctx: &Context, msg: &Message) -> CommandResult {
    let r = reqwest::get("https://randomfox.ca/floof/")
        .await?
        .text()
        .await?;
    let image = &json::parse(r.as_str())?["image"];
    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.colour(0xBBFFFF)
                    .title("Fox")
                    .url("https://randomfox.ca/floof/")
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
