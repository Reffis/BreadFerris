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
        log(Info, &format!("Connected as {}", ready.user.name));
        ctx.set_activity(Activity::playing(format!(
            "ferris help / {} Servers",
            ready.guilds.len()
        )))
        .await;
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        log(Info, "Resumed");
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }
        let x = ctx
            .http
            .get_channels(msg.guild_id.unwrap_or_default().0)
            .await
            .unwrap_or_default();
        let re = regex::Regex::new(r"(.{0,}?discord.gg/.{1,})|(.{0,}?discord.com/invite/.{1,})|(.{0,}?discordapp.com/invite/.{1,})").unwrap();
        for c in x {
            if c.id == msg.channel_id {
                if c.topic.unwrap_or_default().contains("-NoInviteLink") {
                    if re.is_match(&msg.content) {
                        msg.delete(&ctx.http).await.unwrap_or_default();
                        if let Err(_) = msg
                            .channel_id
                            .send_message(&ctx.http, |f| {
                                f.embed(|e| {
                                    e.title("서버 초대 링크 감지됨")
                                        .author(|a| {
                                            a.name(&msg.author.tag()).icon_url(
                                                &msg.author.avatar_url().unwrap_or_default(),
                                            )
                                        })
                                        .description(
                                            r#"
해당 채널에서는 초대 링크를 보낼 수 없습니다.
채널 주제에서 `-NoInviteLink`를 제거하면, 해당 기능이 비활성화됩니다.
                                "#,
                                        )
                                })
                            })
                            .await
                        {}
                    }
                }
            }
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
                log(Error, "ferris / Failed to send message");
            }
        }
    }
}
