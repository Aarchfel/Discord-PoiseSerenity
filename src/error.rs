use crate::{Data, Error, log_error, logger::Logger};
use poise::FrameworkError;

pub async fn on_error(err: FrameworkError<'_, Data, Error>) {
    match err {
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

        FrameworkError::Setup { error, .. } => {
            let msg = format_args!("Failed to setup framework and initialize bot: {:?}", error);

            Logger::new().critical(msg);
        }

        other_error => {
            if let Err(e) = poise::builtins::on_error(other_error).await {
                let msg = format_args!("Failed to handle error: {}", e);

                Logger::new().error(msg);
            }
        }
    }
}
