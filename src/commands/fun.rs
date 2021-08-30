use super::embed_colors::*;
use bbanglog::info;
use rand::prelude::SliceRandom;
use rand::Rng;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[aliases("랜덤", "골라", "random")]
async fn choice(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let r = args.rest().split(", ").collect::<Vec<_>>();
    info!("Command: [{}] {}", &msg.author.id, &msg.content);
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

    info!("Command: [{}] {}", &msg.author.id, &msg.content);
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
    info!("Command: [{}] {}", &msg.author.id, &msg.content);
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
    info!("Command: [{}] {}", &msg.author.id, &msg.content);
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
    info!("Command: [{}] {}", &msg.author.id, &msg.content);
    Ok(())
}

#[command]
#[aliases("빵켓")]
async fn bbangcat(ctx: &Context, msg: &Message) -> CommandResult {
    let m = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.components(|c| {
                c.create_action_row(|af| {
                    af.create_select_menu(|f| {
                        f.placeholder("빵켓은 귀엽나요?")
                            .custom_id("bbang_cute")
                            .options(|o| {
                                o.create_option(|ff| {
                                    ff.label("귀엽습니다.")
                                        .description("세상에서 가장 귀엽고 깜찍하죠 ><")
                                        .value("bbang_cute")
                                })
                                .create_option(|ff| {
                                    ff.label("귀엽지 않습니다.")
                                        .description("지가 귀엽다고 하는게 정말 역겹죠?")
                                        .value("bbang_notcute")
                                })
                                .create_option(|ff| ff.label("꺼지세요").value("bbang_shutup"))
                            })
                    })
                })
            })
            .content(format!(
                "**빵켓이 어떤지 골라주세요!**\n\n**해당 메뉴는 {} 님만 사용할 수 있습니다.**",
                msg.author.mention()
            ))
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
        if let Some(e) = interaction_data.data.values.get(0) {
            let t = e.as_str();
            if t == "bbang_notcute" {
                msg.reply(
                    ctx,
                    format!(
                        "{}, **틀렸어요. `t ^^ t`**, 빵켓은 귀엽답니다.",
                        msg.author.mention()
                    ),
                )
                .await?;
                m.delete(&ctx.http).await?;
                interaction_data
                    .create_interaction_response(ctx, |f| {
                        f.kind(InteractionResponseType::DeferredUpdateMessage)
                    })
                    .await
                    .unwrap_or_default();
            } else if t == "bbang_cute" {
                msg.reply(
                    ctx,
                    format!("{}, 정답입니다. 빵켓은 귀엽습니다.", msg.author.mention()),
                )
                .await?;

                m.delete(&ctx.http).await?;
                interaction_data
                    .create_interaction_response(ctx, |f| {
                        f.kind(InteractionResponseType::DeferredUpdateMessage)
                    })
                    .await
                    .unwrap_or_default();
            } else if t == "bbang_shutup" {
                m.delete(&ctx.http).await?;
            }
        }
    }
    m.delete(&ctx.http).await?;
    info!("Command: [{}] {}", &msg.author.id, &msg.content);
    Ok(())
}

#[command]
#[aliases("궁합")]
async fn gunghab(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let owo = args.single::<String>()?;
    let uwu = args.single::<String>()?;

    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.colour(PURPLE)
                    .title(format!("**{}** 와, **{}** 의 궁합", owo, uwu))
                    .description(format!("**{}%**", rand::thread_rng().gen_range(0..100)))
                    .footer(|f| {
                        f.text(msg.author.id)
                            .icon_url(msg.author.avatar_url().unwrap_or_default())
                    })
            })
        })
        .await?;
    info!("Command: [{}] {}", &msg.author.id, &msg.content);
    Ok(())
}

#[command]
#[aliases("uwu")]
async fn owo(ctx: &Context, msg: &Message) -> CommandResult {
    let e = "OwO, UwU, 0w0, :>, ;), :), ;(, :<, :(, >_<"
        .split(", ")
        .collect::<Vec<&str>>();
    msg.reply(
        ctx,
        format!(
            "`{}`",
            e.choose(&mut rand::thread_rng()).unwrap_or_else(|| &"")
        ),
    )
    .await?;
    info!("Command: [{}] {}", &msg.author.id, &msg.content);
    Ok(())
}

#[command]
#[aliases("스포일러")]
async fn spoiler(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let result = args
        .rest()
        .chars()
        .map(|x| format!("||{}||", x.to_string()))
        .collect::<String>();
    msg.reply(ctx, format!("{result}\n`{result}`", result = result))
        .await?;
    info!("Command: [{}] {}", &msg.author.id, &msg.content);
    Ok(())
}

#[command]
async fn nevergonnagiveyouup(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "https://youtu.be/dQw4w9WgXcQ").await?;
    info!("Command: [{}] {}", &msg.author.id, &msg.content);
    Ok(())
}

#[command]
#[aliases("아이피", "IP")]
async fn ip(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let r = reqwest::get(format!("https://ipinfo.io/{}/json", args.rest()))
        .await?
        .text()
        .await?;
    let json = &json::parse(r.as_str())?;
    if json["status"] == "404" {
        msg.reply(ctx, "잘못된 아이피입니다.").await?;
    } else {
        msg.channel_id
            .send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.colour(random_color())
                        .title(json["ip"].clone())
                        .url(format!("https://{}", json["ip"].clone()))
                        .field("국가", json["country"].clone(), true)
                        .field("도시", json["city"].clone(), true)
                        .field("위치", json["loc"].clone(), true)
                        .field("우편번호", json["postal"].clone(), true)
                        .field("timezone", json["timezone"].clone(), true)
                        .footer(|f| {
                            f.text(format!("{}", msg.author.tag()));
                            f.icon_url(msg.author.avatar_url().unwrap_or_default())
                        })
                })
            })
            .await?;
    }
    info!("Command: [{}] {}", &msg.author.id, &msg.content);
    Ok(())
}
