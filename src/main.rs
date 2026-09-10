use std::sync::Arc;

use chrono::Utc;
use poise::serenity_prelude::{self as serenity, ActivityData, OnlineStatus};
use songbird::SerenityInit;

mod bot;
mod cache;
mod commands;
mod config;
mod db;
mod domain;
mod error;
mod logger;

use bot::Data; // DATA STRUCT GOES TO bot.rs
use config::Settings; // ENV CONFIG ETC GOES TO sconfig/settings.rs
use error::on_error; // ERROR HANDLER ETC GOES TO error.rs

use error::BotError as Error;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[tokio::main]
async fn main() {
    logger::spawn_memory_updater();

    tracing_subscriber::fmt()
        .event_format(logger::CustomLogFormatter)
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    if let Err(err) = run().await {
        tracing::error!("fatal error: {}", err);
        std::process::exit(1);
    }
}

async fn run() -> error::Result<()> {
    let settings = Settings::from_env()?;
    let db = db::postgres::connect(&settings.postgres_url).await?;

    let intents = serenity::GatewayIntents::non_privileged()
        | serenity::GatewayIntents::MESSAGE_CONTENT
        | serenity::GatewayIntents::GUILD_MESSAGES
        | serenity::GatewayIntents::GUILD_MEMBERS; // IMPORTANT TO ALLOW ALL INTENTS ON DISCORD DEV

    let settings_for_setup = settings.clone();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: commands::register_all(),
            on_error: |err| Box::pin(on_error(err)),
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("~".into()),
                edit_tracker: Some(Arc::new(poise::EditTracker::for_timespan(
                    std::time::Duration::from_secs(3600),
                ))),
                ..Default::default()
            },
            ..Default::default()
        })
        .setup(move |ctx, ready_data, framework| {
            Box::pin(async move {
                tracing::info!("Bot is ready! Connected as {}", ready_data.user.name);

                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    db,
                    config_cache: cache::ConfigCache::new(),
                    settings: settings_for_setup,
                    start_time: Utc::now(),
                    http: reqwest::Client::new(),
                })
            })
        })
        .build();

    let mut client = serenity::ClientBuilder::new(&settings.discord_token, intents)
        .framework(framework)
        .register_songbird()
        .status(OnlineStatus::DoNotDisturb)
        .activity(ActivityData::custom("Running in rust, powered by hope"))
        .await
        .expect("Failed to create serenity client");

    client.start().await?;
    Ok(())
}
