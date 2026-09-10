use crate::bot::Data;
use crate::error::BotError;

pub mod admin;
pub mod debug;

// Register All Commands
pub fn register_all() -> Vec<poise::Command<Data, BotError>> {
    let mut commands = Vec::new();

    commands.extend(debug_commands());
    commands.extend(user_commands());
    commands.extend(admin_commands());

    commands
}

fn debug_commands() -> Vec<poise::Command<Data, BotError>> {
    vec![
        debug::ping(),
        // more commands
    ]
}

fn user_commands() -> Vec<poise::Command<Data, BotError>> {
    vec![
        // more commands
    ]
}

fn admin_commands() -> Vec<poise::Command<Data, BotError>> {
    vec![
        admin::setup(),
        // more commands
    ]
}
