use std::fmt::Display;

use bevy_ggrs::ggrs::PlayerType;
use clap::Parser;

use bevy::prelude::*;
use bevy_console::{
    AddConsoleCommand, ConsoleCommand, ConsoleConfiguration, ConsolePlugin, ToggleConsoleKey,
};

use crate::{network::socket::AceSocket, player::PersistentPlayerStats, GameState, RollbackState};

#[derive(Resource, Default)]
pub struct CommandQueue {
    queue: Vec<AceCommands>,
}

pub fn apply_commands(
    mut command_queque: ResMut<CommandQueue>,
    mut stats: ResMut<PersistentPlayerStats>,
) {
    for command in command_queque.queue.drain(..) {
        match command {
            AceCommands::SetMaxSpeed(handle, value) => {
                if handle >= stats.stats.len() {
                    return;
                }

                info!("applying command");
                stats.stats[handle].max_speed = value;
            }
        }
    }
}

pub fn receive_commands(mut socket: ResMut<AceSocket>, mut command_queque: ResMut<CommandQueue>) {
    let received_messages = socket.receive_tcp_message();

    if received_messages.is_empty() {
        return;
    }

    for message in received_messages {
        match AceCommands::from_str(&message.1) {
            Some(ace_command) => {
                info!("received command messsage, adding to queue");
                command_queque.queue.push(ace_command);
            }
            None => warn!("command message failed to pass to string"),
        }
    }
}

pub struct AceConsolePlugin;

impl Plugin for AceConsolePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            receive_commands
                .chain()
                .run_if(in_state(GameState::InRollbackGame)),
        )
        .add_plugins(ConsolePlugin)
        .insert_resource(ConsoleConfiguration {
            keys: vec![ToggleConsoleKey::KeyCode(KeyCode::F1)],
            ..default()
        })
        .init_resource::<CommandQueue>()
        .add_console_command::<SetMaxSpeed, _>(set_max_speed_command)
        .add_systems(OnEnter(RollbackState::InRound), apply_commands);
    }
}

/// Prints given arguments to the console
#[derive(Parser, ConsoleCommand)]
#[command(name = "set-max-speed")]
struct SetMaxSpeed {
    handle: usize,
    value: f32,
}

#[derive(Clone)]
enum AceCommands {
    SetMaxSpeed(usize, f32),
}

impl Display for AceCommands {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SetMaxSpeed(handle, value) => write!(f, "set-max-speed {} {}", handle, value),
        }
    }
}

impl AceCommands {
    fn from_str(s: &str) -> Option<AceCommands> {
        let parts: Vec<&str> = s.split_whitespace().collect();

        if parts.len() < 2 {
            return None;
        }

        match parts[0] {
            "set-max-speed" => {
                if let Ok(handle) = parts[1].parse() {
                    if let Ok(value) = parts[2].parse() {
                        return Some(AceCommands::SetMaxSpeed(handle, value));
                    }
                }
            }
            _ => {}
        }
        None
    }
}

fn set_max_speed_command(
    mut cmd: ConsoleCommand<SetMaxSpeed>,
    mut socket: ResMut<AceSocket>,
    mut command_queque: ResMut<CommandQueue>,
) {
    if let Some(Ok(SetMaxSpeed { handle, value })) = cmd.take() {
        for p in socket.players() {
            if let PlayerType::Remote(peer_id) = p {
                let push_cmd = AceCommands::SetMaxSpeed(handle, value);
                socket.send_tcp_message(peer_id, &push_cmd.to_string());
                command_queque.queue.push(push_cmd.clone());
            };
        }
        cmd.ok();
    }
}
