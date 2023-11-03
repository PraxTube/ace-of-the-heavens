use chrono::Utc;
use clap::Parser;

use bevy::prelude::*;
use bevy_console::{AddConsoleCommand, ConsoleCommand};
use bevy_ggrs::ggrs::PlayerType;

use super::{AceCommands, CommandQueue};
use crate::network::{ggrs_config::PLAYER_COUNT, socket::AceSocket};

pub struct AceCommandPlugin;

impl Plugin for AceCommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_console_command::<Mayhem, _>(mayhem_command)
            .add_console_command::<Buff, _>(buff_command)
            .add_console_command::<Reset, _>(reset_command)
            .add_console_command::<Nerf, _>(nerf_command);
    }
}

/// Let there be mayhem
#[derive(Parser, ConsoleCommand)]
#[command(name = "mayhem")]
pub struct Mayhem;

fn mayhem_command(
    mut cmd: ConsoleCommand<Mayhem>,
    mut socket: ResMut<AceSocket>,
    mut command_queque: ResMut<CommandQueue>,
) {
    if let Some(Ok(Mayhem)) = cmd.take() {
        for p in socket.players() {
            if let PlayerType::Remote(peer_id) = p {
                let timestamp = Utc::now().timestamp_millis();
                for i in 0..2 {
                    let push_cmd = AceCommands::Buff(i, 9, timestamp);
                    socket.send_tcp_message(peer_id, &push_cmd.to_string());
                    command_queque.queue.push(push_cmd.clone());
                }
            };
        }
        cmd.ok();
    }
}

/// Set buff level for one player
#[derive(Parser, ConsoleCommand)]
#[command(name = "buff")]
pub struct Buff {
    /// The handle of the player from 0 to PLAYER_COUNT - 1
    pub handle: usize,
    /// Buff Level from 0 (default, no nerf) to 9 (max buff)
    pub level: usize,
}

fn buff_command(
    mut cmd: ConsoleCommand<Buff>,
    mut socket: ResMut<AceSocket>,
    mut command_queque: ResMut<CommandQueue>,
) {
    if let Some(Ok(Buff { handle, level })) = cmd.take() {
        if handle >= PLAYER_COUNT || level >= 10 {
            cmd.failed();
            return;
        }
        for p in socket.players() {
            if let PlayerType::Remote(peer_id) = p {
                let timestamp = Utc::now().timestamp_millis();
                let push_cmd = AceCommands::Buff(handle, level, timestamp);
                socket.send_tcp_message(peer_id, &push_cmd.to_string());
                command_queque.queue.push(push_cmd.clone());
            };
        }
        cmd.ok();
    }
}

/// Reset back to default
#[derive(Parser, ConsoleCommand)]
#[command(name = "reset")]
pub struct Reset;

fn reset_command(
    mut cmd: ConsoleCommand<Reset>,
    mut socket: ResMut<AceSocket>,
    mut command_queque: ResMut<CommandQueue>,
) {
    if let Some(Ok(Reset)) = cmd.take() {
        for p in socket.players() {
            if let PlayerType::Remote(peer_id) = p {
                let timestamp = Utc::now().timestamp_millis();
                for i in 0..2 {
                    let push_cmd = AceCommands::Nerf(i, 0, timestamp);
                    socket.send_tcp_message(peer_id, &push_cmd.to_string());
                    command_queque.queue.push(push_cmd.clone());
                }
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
                let timestamp = Utc::now().timestamp_millis();
                let push_cmd = AceCommands::Nerf(handle, level, timestamp);
                socket.send_tcp_message(peer_id, &push_cmd.to_string());
                command_queque.queue.push(push_cmd.clone());
            };
        }
        cmd.ok();
    }
}
