use chrono::DateTime;
use serde::{Deserialize, Serialize};
use std::fmt;

// ===================================================================================
//
//                      Audit Category Management Enums type shi
//                          MY FUCKING GOD THIS IS SOOO LONG
// ===================================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AuditCategory {
    Message,
    Member,
    Moderation,
    Voice,
    ChannelRole,
    Server,
}

impl AuditCategory {
    pub fn label(&self) -> &'static str {
        match self {
            AuditCategory::Message => "Message Logs",
            AuditCategory::Member => "Member Logs",
            AuditCategory::Moderation => "Moderation Logs",
            AuditCategory::Voice => "Voice Logs",
            AuditCategory::ChannelRole => "Channel & Role Logs",
            AuditCategory::Server => "Server Logs",
        }
    }

    // Stable string key for DB storage (primary key column)
    pub fn db_key(&self) -> &'static str {
        match self {
            AuditCategory::Message => "msg_log",
            AuditCategory::Member => "member_log",
            AuditCategory::Moderation => "moderation_log",
            AuditCategory::Voice => "voice_log",
            AuditCategory::ChannelRole => "channel_role_log",
            AuditCategory::Server => "server_log",
        }
    }

    pub fn all() -> &'static [AuditCategory] {
        &[
            AuditCategory::Message,
            AuditCategory::Member,
            AuditCategory::Moderation,
            AuditCategory::Voice,
            AuditCategory::ChannelRole,
            AuditCategory::Server,
        ]
    }
}

// Fucking formatter
impl fmt::Display for AuditCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.label())
    }
}

// There they are fucking shits
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AuditEvent {
    // Message Events
    MessageDelete,
    MessageUpdate,
    MessageDeleteBulk,

    // Member Events
    MemberJoin,
    MemberLeave,
    MemberUpdate,
    MemberTimeout, // Could be the same as MemberUpdate(remove it soon ig)

    // Moderation Events
    BanAdd,
    BanRemove,

    // Channel Events
    ChannelCreate,
    ChannelUpdate,
    ChannelDelete,

    // Role Events
    RoleCreate,
    RoleUpdate,
    RoleDelete,

    // Voice Events
    VoiceJoin,
    VoiceLeave,
    VoiceMove,
    VoiceMute,
    VoiceDeafen,
    VoiceStream,
    VoiceCamera,

    // Guild Events
    GuildUpdate,
    EmojiCreate,
    EmojiDelete,
    StickerUpdate,
}

impl AuditEvent {
    pub fn category(&self) -> AuditCategory {
        use AuditEvent::*;

        match self {
            MessageDelete | MessageUpdate | MessageDeleteBulk => AuditCategory::Message,
            MemberJoin | MemberLeave | MemberUpdate | MemberTimeout => AuditCategory::Member,
            BanAdd | BanRemove => AuditCategory::Moderation,
            ChannelCreate | ChannelUpdate | ChannelDelete => AuditCategory::ChannelRole,
            RoleCreate | RoleUpdate | RoleDelete => AuditCategory::ChannelRole,
            VoiceJoin | VoiceLeave | VoiceMove | VoiceMute | VoiceDeafen | VoiceStream
            | VoiceCamera => AuditCategory::Voice,
            GuildUpdate | EmojiCreate | EmojiDelete | StickerUpdate => AuditCategory::Server,
        }
    }

    pub fn label(&self) -> &'static str {
        use AuditEvent::*;
        match self {
            MessageDelete => "Message Delete",
            MessageUpdate => "Message Update",
            MessageDeleteBulk => "Bulk Message Delete",
            MemberJoin => "Member Join",
            MemberLeave => "Member Left",
            MemberUpdate => "Member Update",
            MemberTimeout => "Member Timed Out",
            BanAdd => "Member Banned",
            BanRemove => "Member Unbanned",
            ChannelCreate => "Channel Create",
            ChannelUpdate => "Channel Update",
            ChannelDelete => "Channel Delete",
            RoleCreate => "Role Create",
            RoleUpdate => "Role Update",
            RoleDelete => "Role Delete",
            VoiceJoin => "Voice Join",
            VoiceLeave => "Voice Leave",
            VoiceMove => "Voice Move",
            VoiceMute => "Voice Mute",
            VoiceDeafen => "Voice Deafen",
            VoiceStream => "Voice Stream",
            VoiceCamera => "Voice Camera",
            GuildUpdate => "Guild Update",
            EmojiCreate => "Emoji Create",
            EmojiDelete => "Emoji Delete",
            StickerUpdate => "Sticker Updated",
        }
    }

    pub fn emoji(&self) -> &'static str {
        use AuditEvent::*;
        match self {
            MessageUpdate | MessageDelete | MessageDeleteBulk => "📝",
            MemberJoin | MemberLeave | MemberUpdate | MemberTimeout => "👤",
            BanAdd | BanRemove => "🔨",
            ChannelCreate | ChannelUpdate | ChannelDelete => "📁",
            RoleCreate | RoleUpdate | RoleDelete => "🎭",
            VoiceJoin | VoiceLeave | VoiceMove | VoiceMute | VoiceDeafen | VoiceStream
            | VoiceCamera => "🔊",
            GuildUpdate | EmojiCreate | EmojiDelete | StickerUpdate => "🎉",
        }
    }

    pub fn description(&self) -> &'static str {
        use AuditEvent::*;
        match self {
            MessageDelete => "Logs when a message is deleted",
            MessageUpdate => "Logs when a message is edited",
            MessageDeleteBulk => "Logs bulk message deletions",
            MemberJoin => "Logs when a member joins the server",
            MemberLeave => "Logs when a member leaves the server",
            MemberUpdate => "Logs nickname/role changes on a member",
            MemberTimeout => "Logs timeout actions on a member",
            BanAdd => "Logs bans",
            BanRemove => "Logs unbans",
            ChannelCreate => "Logs channel creation",
            ChannelDelete => "Logs channel deletion",
            ChannelUpdate => "Logs channel setting changes",
            RoleCreate => "Logs role creation",
            RoleDelete => "Logs role deletion",
            RoleUpdate => "Logs role setting changes",
            VoiceJoin => "Logs joining a voice channel",
            VoiceLeave => "Logs leaving a voice channel",
            VoiceMove => "Logs moving between voice channels",
            VoiceMute => "Logs server mute state changes",
            VoiceDeafen => "Logs server deafen state changes",
            VoiceStream => "Logs starting/stopping a stream",
            VoiceCamera => "Logs enabling/disabling camera",
            GuildUpdate => "Logs server setting changes",
            EmojiCreate => "Logs emoji creation",
            EmojiDelete => "Logs emoji deletion",
            StickerUpdate => "Logs sticker changes",
        }
    }

    pub fn implemented(&self) -> bool {
        use AuditEvent::*;
        // TODO: EVERY EVENTS NEEDS TO BE TRUE OR FALSE, IN THIS CASE I WILL MAKE IT ALL FALSE FOR NOW
        /*
        match self {
            MessageDelete | MessageUpdate | MessageDeleteBulk => true,
            MemberJoin | MemberLeave | MemberUpdate | MemberTimeout => true,
            BanAdd | BanRemove => true,
            ChannelCreate | ChannelUpdate | ChannelDelete => true,
            RoleCreate | RoleUpdate | RoleDelete => true,
            VoiceJoin | VoiceLeave | VoiceMove | VoiceMute | VoiceDeafen | VoiceStream | VoiceCamera => true,
            GuildUpdate | EmojiCreate | EmojiDelete | StickerUpdate => true,
        }
        */
        !matches!(self, VoiceStream | VoiceCamera | StickerUpdate)
    }

    // Stable string key for DB storage
    pub fn db_key(&self) -> &'static str {
        use AuditEvent::*;
        match self {
            MessageDelete => "message_delete",
            MessageUpdate => "message_update",
            MessageDeleteBulk => "message_delete_bulk",
            MemberJoin => "member_join",
            MemberLeave => "member_leave",
            MemberUpdate => "member_update",
            MemberTimeout => "member_timeout",
            BanAdd => "ban_add",
            BanRemove => "ban_remove",
            ChannelCreate => "channel_create",
            ChannelDelete => "channel_delete",
            ChannelUpdate => "channel_update",
            RoleCreate => "role_create",
            RoleDelete => "role_delete",
            RoleUpdate => "role_update",
            VoiceJoin => "voice_join",
            VoiceLeave => "voice_leave",
            VoiceMove => "voice_move",
            VoiceMute => "voice_mute",
            VoiceDeafen => "voice_deafen",
            VoiceStream => "voice_stream",
            VoiceCamera => "voice_camera",
            GuildUpdate => "guild_update",
            EmojiCreate => "emoji_create",
            EmojiDelete => "emoji_delete",
            StickerUpdate => "sticker_update",
        }
    }

    pub fn from_db_key(key: &str) -> Option<Self> {
        use AuditEvent::*;
        Some(match key {
            "message_delete" => MessageDelete,
            "message_update" => MessageUpdate,
            "message_delete_bulk" => MessageDeleteBulk,
            "member_join" => MemberJoin,
            "member_leave" => MemberLeave,
            "member_update" => MemberUpdate,
            "member_timeout" => MemberTimeout,
            "ban_add" => BanAdd,
            "ban_remove" => BanRemove,
            "channel_create" => ChannelCreate,
            "channel_delete" => ChannelDelete,
            "channel_update" => ChannelUpdate,
            "role_create" => RoleCreate,
            "role_delete" => RoleDelete,
            "role_update" => RoleUpdate,
            "voice_join" => VoiceJoin,
            "voice_leave" => VoiceLeave,
            "voice_move" => VoiceMove,
            "voice_mute" => VoiceMute,
            "voice_deafen" => VoiceDeafen,
            "voice_stream" => VoiceStream,
            "voice_camera" => VoiceCamera,
            "guild_update" => GuildUpdate,
            "emoji_create" => EmojiCreate,
            "emoji_delete" => EmojiDelete,
            "sticker_update" => StickerUpdate,
            _ => return None,
        })
    }

    pub fn all() -> &'static [AuditEvent] {
        use AuditEvent::*;
        &[
            // Message
            MessageDelete,
            MessageUpdate,
            MessageDeleteBulk,
            // Member
            MemberJoin,
            MemberLeave,
            MemberUpdate,
            MemberTimeout,
            // Moderation
            BanAdd,
            BanRemove,
            // Channel
            ChannelCreate,
            ChannelDelete,
            ChannelUpdate,
            // Role
            RoleCreate,
            RoleDelete,
            RoleUpdate,
            // Voice
            VoiceJoin,
            VoiceLeave,
            VoiceMove,
            VoiceMute,
            VoiceDeafen,
            VoiceStream,
            VoiceCamera,
            // Guild
            GuildUpdate,
            EmojiCreate,
            EmojiDelete,
            StickerUpdate,
        ]
    }
}

// Normalized audit entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub guild_id: u64,
    pub event: AuditEvent,
    pub actor_id: Option<u64>,
    pub target_id: Option<u64>,
    pub reason: Option<String>,
    pub metadata: serde_json::Value,
    pub timestamp: DateTime<chrono::Utc>,
}
