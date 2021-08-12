use super::embed_colors::*;
use bbangcat_encryption::bce::*;
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
    msg.reply(ctx, format!("`discord.gift/{}`\n\n`주의!`: `해당 코드는 작동되지 않는 코드입니다. 그냥 재미로만 해주세요 :)`", v)).await?;

    cmdlog(msg.author.id.to_string(), msg.content.clone());
    Ok(())
}

#[command]
#[aliases("빵암호화", "빵켓암호화")]
async fn bce(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    msg.reply(ctx, to_bce::new(args.rest())).await?;
    cmdlog(msg.author.id.to_string(), msg.content.clone());
    Ok(())
}

#[command]
#[aliases("say", "따라해")]
async fn say(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.colour(PURPLE)
                    .title(format!("{}", args.rest()))
                    .footer(|f| {
                        f.text(msg.author.id)
                            .icon_url(msg.author.avatar_url().unwrap_or_default())
                    })
            })
        })
        .await?;
    cmdlog(msg.author.id.to_string(), msg.content.clone());
    Ok(())
}

#[command]
#[aliases("아바타", "profile", "프로필")]
async fn avatar(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let user_id = args
        .single::<String>()?
        .replace("<", "")
        .replace(">", "")
        .replace("@", "")
        .replace("!", "")
        .parse::<u64>()?;
    let user = ctx.http.get_user(user_id).await?;
    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.colour(random_color())
                    .title(format!(
                        "{}님의 아바타입니다.",
                        format!("{}#{}", user.name, user.discriminator)
                    ))
                    .url(user.avatar_url().unwrap_or_default())
                    .image(user.avatar_url().unwrap_or_default())
            })
        })
        .await?;
    cmdlog(msg.author.id.to_string(), msg.content.clone());
    Ok(())
}

#[command]
#[aliases("한강")]
async fn hangang(ctx: &Context, msg: &Message) -> CommandResult {
    let r = reqwest::get("https://api.hangang.msub.kr/")
        .await?
        .text()
        .await?;
    let v = &json::parse(r.as_str())?;
    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.colour(WHITE)
                    .title("한강 수온")
                    .url("https://hangang.msub.kr")
                    .description(format!("**{}**", v["temp"]))
                    .footer(|f| f.text(format!("{} - {}", v["time"], v["station"])))
            })
        })
        .await?;
    cmdlog(msg.author.id.to_string(), msg.content.clone());
    Ok(())
}
