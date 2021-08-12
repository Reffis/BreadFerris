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
    let m = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.components(|c| {
                c.create_action_row(|a| {
                    a.create_button(|b| b.label("hi").style(ButtonStyle::Success).custom_id("asdf"))
                        .create_button(|b| {
                            b.label("bye").style(ButtonStyle::Success).custom_id("abc")
                        })
                })
            })
            .embed(|x| x.title("asdf"))
        })
        .await
        .unwrap();

    if let Some(interaction_data) = m
        .await_component_interaction(ctx)
        .author_id(msg.author.id.0)
        .timeout(std::time::Duration::from_secs(60))
        .await
    {
        if interaction_data.data.custom_id == "asdf" {
            msg.reply(ctx, "sans").await?;
            interaction_data
                .create_interaction_response(ctx, |x| x)
                .await?;
            interaction_data.delete_followup_message(ctx, m.id).await?;
        } else if interaction_data.data.custom_id == "abc" {
            msg.reply(ctx, "wa").await?;
            interaction_data
                .create_interaction_response(ctx, |x| x)
                .await?;
            interaction_data.delete_followup_message(ctx, m.id).await?;
        }
    }

    Ok(())
}
