use breadferris::{log, LogType::*};
use serenity::model::channel::Message;
use serenity::model::gateway::Activity;
use serenity::{
    async_trait,
    model::{event::ResumedEvent, gateway::Ready},
    prelude::*,
};

use super::commands::embed_colors::*;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        log(Info, format!("Connected as {}", ready.user.name));
        ctx.set_activity(Activity::playing(format!(
            "ferris help / {} Servers",
            ready.guilds.len()
        )))
        .await;
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        log(Info, "Resumed".to_string());
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }
        if msg.content.to_lowercase() == "ferris" {
            if let Err(_) = msg.channel_id.send_message(&ctx.http, |m| {
                m.embed(|embed| {
                    embed.colour(random_color())
                    .title("Ferris Bot")
                    .description(r#"
**페리스 봇입니다!**

도움말은 `ferris help` 를 입력하여, 확인해주세요.

또한, 페리스 봇의 모든것은 오픈소스입니다!
(`ferris help` 로 확인해주세요.)

**- `! Bread Cat#0002` -**
                    "#)
                    .thumbnail("https://cdn.discordapp.com/attachments/850930041487622197/878290746912960542/bot.png")
                })
            }).await {
                log(Error, "ferris / Failed to send message".to_string());
            }
        }
    }
}
