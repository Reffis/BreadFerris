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
    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.components(|c| {
                c.create_action_row(|a| {
                    a.create_button(|b| b.label("hi").style(ButtonStyle::Success).custom_id("asdf"))
                })
            })
            .embed(|x| x.title("asdf"))
        })
        .await
        .unwrap();

    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.components(|c| {
                c.create_action_row(|a| {
                    a.create_select_menu(|m| {
                        m.custom_id("asdfasdf").options(|o| {
                            o.create_option(|xx| {
                                xx.label("ㅁㄴㅇㄻㄴㅇㄹ").description("샍").value("헋")
                            })
                        })
                    })
                })
            })
            .embed(|x| x.title("asdf"))
        })
        .await
        .unwrap();
    Ok(())
}
