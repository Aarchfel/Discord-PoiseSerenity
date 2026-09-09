use chrono::{DateTime, Utc};
use poise::serenity_prelude::{self as serenity, ActivityData, OnlineStatus};
use songbird::SerenityInit;
use std::{env, sync::Arc};

mod commands;
mod db;
mod error;
mod logger;

use error::on_error;

pub struct Data {
    pub start_time: DateTime<Utc>,
    pub http: reqwest::Client,
}

type Error = Box<dyn std::error::Error + Send + Sync>;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN cannot be found in .env!");

    let framework = poise::Framework::<Data, Error>::builder()
        .options(poise::FrameworkOptions {
            commands: commands::register_all(),

            on_error: |error| Box::pin(on_error(error)),

            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("~".into()),
                edit_tracker: Some(Arc::new(poise::EditTracker::for_timespan(
                    std::time::Duration::from_secs(3600),
                ))),
                ..Default::default()
            },
            ..Default::default()
        })
        .setup(|ctx, ready_data, framework| {
            Box::pin(async move {
                log_info!("Bot is ready! Connected as {}", ready_data.user.name);

                poise::builtins::register_globally(ctx, &framework.options().commands).await?;

                let cmd_count = framework.options().commands.len();

                log_info!("Loaded {} command(s) globally", cmd_count);

                for cmd in &framework.options().commands {
                    log_debug!(
                        "  - Command: /{} (subcommands: {})",
                        cmd.name,
                        cmd.subcommands.len()
                    );
                }

                Ok(Data {
                    start_time: Utc::now(),
                    http: reqwest::Client::new(),
                })
            })
        })
        .build();

    let intents = serenity::GatewayIntents::all();

    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .register_songbird()
        .status(OnlineStatus::DoNotDisturb)
        .activity(ActivityData::custom("Running in rust, powered by hope"))
        .await
        .expect("Failed to create serenity client");

    if let Err(why) = client.start().await {
        log_critical!("Client error: {}", why);
    }
}
