use crate::{Data, Error};

type Context<'a> = poise::Context<'a, Data, Error>;

/// Ping this bot's latency
#[poise::command(slash_command, prefix_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let latency = ctx.ping().await;

    ctx.say(format!("Pong! Latency: {}ms", latency.as_millis()))
        .await?;

    Ok(())
}
