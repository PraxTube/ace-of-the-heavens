pub use commands::AceCommandPlugin;

mod commands;

use std::fmt::Display;

use bevy::prelude::*;

use super::CommandQueue;
use crate::{network::socket::AceSocket, player::PersistentPlayerStats};

#[derive(Clone)]
pub enum AceCommands {
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
    pub fn from_str(s: &str) -> Option<AceCommands> {
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
            "dummy" => {}
            _ => {}
        }
        None
    }
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
