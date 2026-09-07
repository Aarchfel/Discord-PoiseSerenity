use crate::{Data, Error, log_critical, log_error};
use poise::FrameworkError;

pub async fn on_error(err: FrameworkError<'_, Data, Error>) {
    match err {
        // Command error
        FrameworkError::Command { error, ctx, .. } => {
            let user = &ctx.author().name;
            let command_name = &ctx.command().name;

            log_error!(
                "Error on command /'{}' (triggered by {}): {}",
                command_name,
                user,
                error
            );

            let _ = ctx.say(format!("Failed to run command: {}", error)).await;
        }

        // Argument Error
        FrameworkError::ArgumentParse {
            error, ctx, input, ..
        } => {
            log_error!("Error on argument: {}", error);

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
            log_critical!("Failed to setup framework and initialize bot: {:?}", error);
        }

        // Cooldown Handler
        FrameworkError::CooldownHit {
            remaining_cooldown,
            ctx,
            ..
        } => {
            let sec_left = remaining_cooldown.as_secs_f32();

            let resp = format!(
                "You must wait {:.2} seconds before using this command again.",
                sec_left
            );

            if let Err(e) = ctx.say(resp).await {
                log_error!("Failed to send cooldown message: {:?}", e);
            }
        }

        // Other Error Handler
        other_error => {
            if let Err(e) = poise::builtins::on_error(other_error).await {
                log_error!("Failed to handle error: {:?}", e);
            }
        }
    }
}
