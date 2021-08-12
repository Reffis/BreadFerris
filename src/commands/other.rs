use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use breadferris::cmdlog;
use serenity::model::interactions::message_component::ButtonStyle;

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
                    a.create_button(|b| b.label("hi").style(ButtonStyle::Success).custom_id("asdf"))
                        .create_button(|b| {
                            b.label("bye").style(ButtonStyle::Success).custom_id("abc")
                        })
                        .create_button(|b| {
                            b.label("Delete")
                                .style(ButtonStyle::Danger)
                                .custom_id("del")
                        })
                })
            })
            .embed(|x| x.title("asdf"))
        })
        .await
        .unwrap();
    loop {
        if let Some(interaction_data) = m
            .await_component_interaction(ctx)
            .author_id(msg.author.id.0)
            .await
        {
            if interaction_data.data.custom_id == "asdf" {
                m.edit(&ctx.http, |f| f.embed(|x| x.title("헬로"))).await?;
                interaction_data
                    .create_interaction_response(ctx, |f| f)
                    .await
                    .unwrap_or_default();
            } else if interaction_data.data.custom_id == "abc" {
                m.edit(&ctx.http, |f| f.embed(|x| x.title("월드"))).await?;
                interaction_data
                    .create_interaction_response(ctx, |f| f)
                    .await
                    .unwrap_or_default();
            } else if interaction_data.data.custom_id == "del" {
                m.delete(&ctx.http).await?;
                break;
            }
        }
    }
    Ok(())
}
