use std::collections::HashMap;

use crate::domain::AuditCategory;

#[derive(Debug, Clone, Default)]
pub struct GuildConfig {
    pub guild_id: u64,

    pub audit_enabled: bool,
    pub audit_default_channel: Option<u64>,
    pub audit_category_channels: HashMap<AuditCategory, u64>,

    // event db_key -> (enabled, ping_role_id)
    pub audit_event_overrides: HashMap<String, (bool, Option<u64>)>,

    // AI slop
    pub ai_moderation_enabled: bool,
}

impl GuildConfig {
    pub fn new(guild_id: u64) -> Self {
        Self {
            guild_id,
            ..Default::default()
        }
    }
}
