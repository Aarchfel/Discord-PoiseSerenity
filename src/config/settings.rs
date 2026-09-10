use crate::error::{BotError, Result};

#[derive(Debug, Clone)]
pub struct Settings {
    pub discord_token: String,
    pub postgres_url: String,
    pub sqlite_path: String,
    pub max_ai_workers: usize,
}

impl Settings {
    pub fn from_env() -> Result<Self> {
        dotenvy::dotenv().ok();

        let discord_token = env_var("DISCORD_TOKEN")?;
        let postgres_url = env_var("DATABASE_URL")?;
        let sqlite_path =
            std::env::var("SQLITE_PATH").unwrap_or_else(|_| "./data/cache.sqlite".to_string());
        let max_ai_workers = std::env::var("MAX_AI_WORKERS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(3);

        Ok(Self {
            discord_token,
            postgres_url,
            sqlite_path,
            max_ai_workers,
        })
    }
}

fn env_var(key: &str) -> Result<String> {
    std::env::var(key).map_err(|_| BotError::Config(format!("missing required env var: {key}")))
}
