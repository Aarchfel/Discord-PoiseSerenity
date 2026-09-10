use crate::domain::GuildConfig;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

// L1 Cache perguild configuration held in ram
// paths never hit postgres
#[derive(Clone, Default)]
pub struct ConfigCache {
    inner: Arc<RwLock<HashMap<u64, GuildConfig>>>, // TODO: Make GuildConfig Domain
}

impl ConfigCache {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn get(&self, guild_id: u64) -> Option<GuildConfig> {
        self.inner.read().await.get(&guild_id).cloned()
    }

    pub async fn set(&self, guild_id: u64, config: GuildConfig) {
        self.inner.write().await.insert(guild_id, config);
    }

    pub async fn invalidate(&self, guild_id: u64) {
        self.inner.write().await.remove(&guild_id);
    }
}
