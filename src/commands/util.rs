use super::image_lib::*;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::time::Instant;

use image::imageops::FilterType;

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
                    .field(
                        "Utility",
                        r#"
```diff
help
ping
support [Message]
```
                    "#,
                        true,
                    )
                    .field(
                        "Owner",
                        r#"
```diff
eval [Code]
quit
status [Message]
```
                    "#,
                        true,
                    )
                    .field(
                        "Image",
                        r#"
```diff
fox
shiba
cat
```
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

    Ok(())
}

// TODO
#[command]
async fn image(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    println!("{}", msg.author.avatar_url().unwrap_or_default());
    fetch_url(args.rest().to_string(), "test.png".to_string()).await?;
    use image::open;
    let img = open("test.png").unwrap();
    img.resize(200, 200, FilterType::Nearest)
        .save("temp.png")
        .unwrap();
    msg.channel_id
        .send_message(&ctx.http, |m| m.add_file("temp.png"))
        .await?;

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
            let channel = ctx.http.get_channel(873747323266666497).await?;
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

    Ok(())
}