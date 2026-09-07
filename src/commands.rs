use crate::{Data, Error};

pub mod debug;

// Register All Commands
pub fn register_all() -> Vec<poise::Command<Data, Error>> {
    let mut commands = Vec::new();

    commands.extend(debug_commands());
    commands.extend(user_commands());
    commands.extend(admin_commands());

    commands
}

fn debug_commands() -> Vec<poise::Command<Data, Error>> {
    vec![
        debug::ping(),
        // more commands
    ]
}

fn user_commands() -> Vec<poise::Command<Data, Error>> {
    vec![
        // more commands
    ]
}

fn admin_commands() -> Vec<poise::Command<Data, Error>> {
    vec![
        // more commands
    ]
}
