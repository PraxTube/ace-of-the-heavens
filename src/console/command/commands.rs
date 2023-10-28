use clap::Parser;

use bevy::prelude::*;
use bevy_console::{AddConsoleCommand, ConsoleCommand};
use bevy_ggrs::ggrs::PlayerType;

use super::{AceCommands, CommandQueue};
use crate::network::{ggrs_config::PLAYER_COUNT, socket::AceSocket};

pub struct AceCommandPlugin;

impl Plugin for AceCommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_console_command::<SetMaxSpeed, _>(set_max_speed_command)
            .add_console_command::<Nerf, _>(nerf_command);
    }
}

/// Set max speed value of one player
#[derive(Parser, ConsoleCommand)]
#[command(name = "set-max-speed")]
pub struct SetMaxSpeed {
    /// The handle of the player from 0 to PLAYER_COUNT - 1
    pub handle: usize,
    /// The new value (must be bigger then MIN_SPEED)
    pub value: f32,
}

fn set_max_speed_command(
    mut cmd: ConsoleCommand<SetMaxSpeed>,
    mut socket: ResMut<AceSocket>,
    mut command_queque: ResMut<CommandQueue>,
) {
    if let Some(Ok(SetMaxSpeed { handle, value })) = cmd.take() {
        if handle >= PLAYER_COUNT {
            cmd.failed();
            return;
        }
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

/// Set nerf level for one player
#[derive(Parser, ConsoleCommand)]
#[command(name = "nerf")]
pub struct Nerf {
    /// The handle of the player from 0 to PLAYER_COUNT - 1
    pub handle: usize,
    /// Nerf Level from 0 (default, no nerf) to 9 (max nerf)
    pub level: usize,
}

fn nerf_command(
    mut cmd: ConsoleCommand<Nerf>,
    mut socket: ResMut<AceSocket>,
    mut command_queque: ResMut<CommandQueue>,
) {
    if let Some(Ok(Nerf { handle, level })) = cmd.take() {
        if handle >= PLAYER_COUNT || level >= 10 {
            cmd.failed();
            return;
        }
        for p in socket.players() {
            if let PlayerType::Remote(peer_id) = p {
                let push_cmd = AceCommands::Nerf(handle, level);
                socket.send_tcp_message(peer_id, &push_cmd.to_string());
                command_queque.queue.push(push_cmd.clone());
            };
        }
        cmd.ok();
    }
}
