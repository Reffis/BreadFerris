use breadferris::{log, LogType::*};
use serenity::model::gateway::Activity;
use serenity::{
    async_trait,
    model::{event::ResumedEvent, gateway::Ready},
    prelude::*,
};

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
        log(Info,"Resumed".to_string());
    }
}
