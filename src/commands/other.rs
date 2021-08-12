use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use breadferris::cmdlog;
use serenity::model::interactions::message_component::ButtonStyle;
use crate::commands::*;
use crate::commands::embed_colors::random_color;

#[command]
#[aliases("샍", "샌즈", "샍즈", "샌주", "샍주")]
async fn sans(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "헋 샍주 아시는구나!").await?;
    cmdlog(msg.author.id.to_string(), msg.content.clone());
    Ok(())
}

// TODO:  Button & Menu Test

#[command]
async fn button(ctx: &Context, msg: &Message) -> CommandResult {
    let mut m = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.components(|c| {
                c.create_action_row(|a| {
                    a.create_button(|b| {
                        b.label("Utility")
                            .style(ButtonStyle::Success)
                            .custom_id("HELP_UTIL")
                    })
                    .create_button(|b| {
                        b.label("IMAGE")
                            .style(ButtonStyle::Success)
                            .custom_id("HELP_IMAGE")
                    })
                    .create_button(|b| {
                        b.label("Moderator")
                            .style(ButtonStyle::Success)
                            .custom_id("HELP_MODER")
                    })
                })
                .create_action_row(|bbb| {
                    bbb.create_button(|b| {
                        b.label("FUN")
                            .style(ButtonStyle::Success)
                            .custom_id("HELP_FUN")
                    })
                    .create_button(|b| {
                        b.label("OWNER")
                            .style(ButtonStyle::Success)
                            .custom_id("HELP_OWNER")
                    })
                    .create_button(|b| {
                        b.label("Delete")
                            .style(ButtonStyle::Danger)
                            .custom_id("HELP_DEL")
                    })
                })
            })
            .embed(|x| {
                x.title("Help")
                    .description("아래의 버튼을 눌러 도움말을 확인하세요.\n\n버튼이 표시되지않나요? 저런.. 이슈를 넣어주세요.")
                    .colour(random_color())
                    .footer(|f| {
                        f.text("OpenSource: https://github.com/Reffis/breadferris");
                        f.icon_url("https://avatars.githubusercontent.com/u/88228766?s=200&v=4")
                    })
            })
        })
        .await
        .unwrap();
    loop {
        if let Some(interaction_data) = m
            .await_component_interaction(ctx)
            .author_id(msg.author.id.0)
            .await
        {
            let t = interaction_data.data.custom_id.as_str();
            if t == "HELP_UTIL" {
                m.edit(&ctx.http, |f| f.embed(|x| {
                    x.title("🎈 - Utility")
                        .description(HELP_UTIL)
                        .colour(random_color())
                        .footer(|f| {
                            f.text("OpenSource: https://github.com/Reffis/breadferris");
                            f.icon_url("https://avatars.githubusercontent.com/u/88228766?s=200&v=4")
                        })
                })).await?;
                interaction_data
                    .create_interaction_response(ctx, |f| f)
                    .await
                    .unwrap_or_default();
            } else if t == "HELP_IMAGE" {
                m.edit(&ctx.http, |f| f.embed(|x| {
                    x.title("🖼️ - Image")
                        .description(HELP_IMAGE)
                        .colour(random_color())
                        .footer(|f| {
                            f.text("OpenSource: https://github.com/Reffis/breadferris");
                            f.icon_url("https://avatars.githubusercontent.com/u/88228766?s=200&v=4")
                        })
                })).await?;
                interaction_data
                    .create_interaction_response(ctx, |f| f)
                    .await
                    .unwrap_or_default();
            } else if t == "HELP_MODER" {
                m.edit(&ctx.http, |f| f.embed(|x| {
                    x.title("🛠️ - Moderator")
                        .description(HELP_MODER)
                        .colour(random_color())
                        .footer(|f| {
                            f.text("OpenSource: https://github.com/Reffis/breadferris");
                            f.icon_url("https://avatars.githubusercontent.com/u/88228766?s=200&v=4")
                        })
                })).await?;
                interaction_data
                    .create_interaction_response(ctx, |f| f)
                    .await
                    .unwrap_or_default();
            } else if t == "HELP_OWNER" {
                m.edit(&ctx.http, |f| f.embed(|x| {
                    x.title("🛡️ - Owner")
                        .description(HELP_OWNER)
                        .colour(random_color())
                        .footer(|f| {
                            f.text("OpenSource: https://github.com/Reffis/breadferris");
                            f.icon_url("https://avatars.githubusercontent.com/u/88228766?s=200&v=4")
                        })
                })).await?;
                interaction_data
                    .create_interaction_response(ctx, |f| f)
                    .await
                    .unwrap_or_default();
            } else if t == "HELP_DEL" {
                m.delete(&ctx.http).await?;
                break;
            } else if t == "HELP_FUN" {
                m.edit(&ctx.http, |f| f.embed(|x| {
                    x.title("🧊 - FUN")
                        .description(HELP_FUN)
                        .colour(random_color())
                        .footer(|f| {
                            f.text("OpenSource: https://github.com/Reffis/breadferris");
                            f.icon_url("https://avatars.githubusercontent.com/u/88228766?s=200&v=4")
                        })
                })).await?;
                interaction_data
                    .create_interaction_response(ctx, |f| f)
                    .await
                    .unwrap_or_default();
            }
        }
    }
    Ok(())
}
