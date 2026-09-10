use crate::cache::ConfigCache;
use crate::config::Settings;
use chrono::{DateTime, Utc};
use sqlx::PgPool;

// No more logic, only data >:3
pub struct Data {
    pub start_time: DateTime<Utc>,
    pub http: reqwest::Client,
    pub db: PgPool,
    pub config_cache: ConfigCache,
    pub settings: Settings,
}

pub type PoiseContext<'a> = poise::Context<'a, Data, crate::error::BotError>;
