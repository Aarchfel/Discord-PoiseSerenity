use crate::Data;
use poise::FrameworkError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BotError {
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("serenity error: {0}")]
    Serenity(#[from] serenity::Error),

    #[error("config error: {0}")]
    Config(String),

    #[error("audit error: {0}")]
    Audit(String),

    #[error("moderation error: {0}")]
    Moderation(String),

    #[error("not found: {0}")]
    NotFound(String),

    #[error("{0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, BotError>;
pub type CommandResult = std::result::Result<(), BotError>;

pub async fn on_error(err: FrameworkError<'_, Data, BotError>) {
    match err {
        // Command error
        FrameworkError::Command { error, ctx, .. } => {
            let user = &ctx.author().name;
            let command_name = &ctx.command().name;

            tracing::error!(
                "Error on command /'{}' (triggered by {}): {}",
                command_name,
                user,
                error
            );

            let error_msg = match &error {
                BotError::Database(_) => {
                    "</> A database error occurred. Please try again later.".to_string()
                }
                BotError::Serenity(_) => "</> Internal Discord API error occurred.".to_string(),
                BotError::NotFound(msg) => format!("</> Not found: {}", msg),
                BotError::Config(msg) => format!("</> Configuration error: {}", msg),
                BotError::Audit(msg) => format!("</> Audit error: {}", msg),
                BotError::Moderation(msg) => format!("</> Moderation error: {}", msg),
                BotError::Other(msg) => format!("</> Error: {}", msg),
            };

            let _ = ctx.say(error_msg).await;
        }

        // Argument Error
        FrameworkError::ArgumentParse {
            error, ctx, input, ..
        } => {
            tracing::error!("Error on argument: {}", error);

            let _ = ctx
                .say(format!(
                    "[ Err ] Argument '{}' is not valid: {}",
                    input.unwrap_or_default(),
                    error
                ))
                .await;
        }

        // Setup Error
        FrameworkError::Setup { error, .. } => {
            tracing::error!("Failed to setup framework and initialize bot: {:?}", error);
        }

        // Cooldown Handler
        FrameworkError::CooldownHit {
            ctx,
            remaining_cooldown,
            ..
        } => {
            let resp = format!(
                "You must wait {:.2} seconds before using this command again.",
                remaining_cooldown.as_secs_f32()
            );

            if let Err(e) = ctx.say(resp).await {
                tracing::error!("Failed to send cooldown message: {:?}", e);
            }
        }

        // Missing User Perms Handler
        FrameworkError::MissingUserPermissions {
            ctx,
            missing_permissions,
            ..
        } => {
            let msg = format!(
                "You are __missing__ the following permission(s) to run this command: {:?}",
                missing_permissions
            );

            if let Err(e) = ctx.say(msg).await {
                tracing::error!("Failed to send missing user permissions response: {:?}", e);
            }
        }

        // Missing Bot Perms Handler
        FrameworkError::MissingBotPermissions {
            missing_permissions,
            ctx,
            ..
        } => {
            let msg = format!(
                "I am __missing__ the following permission(s) to run this command: {:?}",
                missing_permissions
            );

            if let Err(e) = ctx.say(msg).await {
                tracing::error!("Failed to send missing bot permissions response: {:?}", e);
            }
        }

        // Other Error Handler
        other_error => {
            if let Err(e) = poise::builtins::on_error(other_error).await {
                tracing::error!("Failed to handle framework error: {:?}", e);
            }
        }
    }
}
