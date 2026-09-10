use crate::{Data, Error, error::CommandResult};

// TODO: FIX THIS COMMAND, MAKE IT LOOKS LIKE setup.rs
type Context<'a> = poise::Context<'a, Data, Error>;

/// Ping this bot's latency
#[poise::command(slash_command, prefix_command, user_cooldown = 10)] // https://docs.rs/poise/0.6.2/poise/macros/attr.command.html
pub async fn ping(ctx: Context<'_>) -> CommandResult {
    let latency = ctx.ping().await;

    ctx.send(
        poise::CreateReply::default()
            .content(format!("Pong! Latency: {:.2}ms", latency.as_millis()))
            .ephemeral(true),
    )
    .await?;

    // This if you want to auto delete original command messages
    /*
    if let poise::Context::Prefix(ctx) = ctx {
        let _ = ctx.msg.delete(ctx).await;
    }
    */

    Ok(())
}
