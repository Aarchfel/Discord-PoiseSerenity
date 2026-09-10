use crate::bot::Data;
use crate::error::{BotError, CommandResult};

type Context<'a> = poise::Context<'a, Data, BotError>;

/// Setup your server configurations
#[poise::command(
    slash_command,
    prefix_command,
    user_cooldown = 10,
    guild_only,
    required_permissions = "MANAGE_GUILD" // TO CHECK PERMS YOU CAN SEE IT WITH AUTOCOMPLETE POISE
                                          // SERENITY PRELUDE PERMISSIONS MODULES
)]
pub async fn setup(ctx: Context<'_>) -> CommandResult {
    ctx.send(poise::CreateReply::default().content("Setup your server configurations - TODO"))
        .await?;

    Ok(())
}
