use chrono::Utc;
pub use commands::AceCommandPlugin;

mod commands;
mod player_nerf_stats;

use std::fmt::Display;

use bevy::prelude::*;

use super::CommandQueue;
use crate::{network::socket::AceSocket, player::PersistentPlayerStats};

// Milliseconds that must have past since the command was pushed to the queue
// in order to get applied. This is to prevent desyncs.
const MIN_TIME_THRESHOLD: i64 = 1500;

#[derive(Clone)]
pub enum AceCommands {
    Nerf(usize, usize, i64),
}

impl Display for AceCommands {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nerf(handle, level, timestamp) => {
                write!(f, "nerf {} {} {}", handle, level, timestamp)
            }
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
            "nerf" => {
                if let Ok(handle) = parts[1].parse() {
                    if let Ok(level) = parts[2].parse() {
                        if let Ok(timestamp) = parts[3].parse() {
                            return Some(AceCommands::Nerf(handle, level, timestamp));
                        }
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
    let mut skipped_queue = Vec::<AceCommands>::default();
    for command in command_queque.queue.drain(..) {
        match command {
            AceCommands::Nerf(handle, level, timestamp) => {
                if Utc::now().timestamp_millis() - timestamp < MIN_TIME_THRESHOLD {
                    warn!("skipping this command as it was added too late");
                    skipped_queue.push(AceCommands::Nerf(handle, level, timestamp));
                    continue;
                }
                info!("applying nerf command");
                let nerfed_stats = player_nerf_stats::nerf_stats(level);
                stats.stats[handle] = nerfed_stats;
            }
        }
    }

    command_queque.queue.append(&mut skipped_queue);
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

pub fn reset_commands(
    mut command_queque: ResMut<CommandQueue>,
    mut stats: ResMut<PersistentPlayerStats>,
) {
    *command_queque = CommandQueue::default();
    *stats = PersistentPlayerStats::default();
}
