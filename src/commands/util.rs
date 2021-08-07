use serenity::framework::standard::{macros::command, CommandResult, Args};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::time::Instant;
use super::image::*;

use image::imageops::FilterType;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    let api_latency = {
        let instant = Instant::now();
        msg.channel_id.broadcast_typing(&ctx.http).await?;
        instant.elapsed().as_millis() as f64
    };

    msg.reply(ctx, format!("Pong! 🏓\nAPI Latency: {}ms", api_latency))
        .await?;

    Ok(())
}

#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.colour(0x00ff00)
                    .title("Help")
                    .description("prefix: `ferris`")
                    .field("Utility", r#"
```
help
ping
```
                    "#, true)
                    .field("Owner", r#"
```
eval [Arg]
```
                    "#, true)
                    .footer(|f| {
                        f.text("OpenSource: https://github.com/Reffis/breadferris");
                        f.icon_url("https://avatars.githubusercontent.com/u/88228766?s=60&v=4")
                    })
            })
        })
        .await?;

    Ok(())
}

// TODO
#[command]
async fn image(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    println!("{}", msg.author.avatar_url().unwrap_or_default());
    fetch_url(args.rest().to_string(), "test.png".to_string()).await?;
    use image::open;
    let img = open("test.png").unwrap();
    img.resize(200, 200, FilterType::Nearest).save("temp.png").unwrap();
    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.add_file("temp.png")
        })
        .await?;

    Ok(())
}