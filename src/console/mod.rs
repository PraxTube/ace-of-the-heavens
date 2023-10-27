use bevy::prelude::*;
use bevy_ggrs::ggrs::PlayerType;

use crate::{network::socket::AceSocket, player::Player, GameState, RollbackState};

#[derive(Resource, Default)]
pub struct CommandQueue {
    early_queue: Vec<String>,
    queue: Vec<String>,
}

pub fn push_command(
    mut socket: ResMut<AceSocket>,
    mut command_queque: ResMut<CommandQueue>,
    keys: Res<Input<KeyCode>>,
) {
    if !keys.just_pressed(KeyCode::U) {
        return;
    }

    for p in socket.players() {
        if let PlayerType::Remote(peer_id) = p {
            info!("sending cmd message, adding to own queue");
            socket.send_tcp_message(peer_id, "lower max_speed");
            command_queque.queue.push("lower max_speed".to_string());
        };
    }
}

pub fn receive_command(mut socket: ResMut<AceSocket>, mut command_queque: ResMut<CommandQueue>) {
    let received_messages = socket.receive_tcp_message();

    if received_messages.is_empty() {
        return;
    }

    for message in received_messages {
        if message.1 == "lower max_speed".to_string() {
            info!("received command messsage, adding to queue");
            command_queque.queue.push("lower max_speed".to_string());
        }
    }
}

pub fn apply_commands(mut command_queque: ResMut<CommandQueue>, mut players: Query<&mut Player>) {
    for command in command_queque.early_queue.drain(..) {
        if &command == "lower max_speed" {
            for mut player in &mut players {
                if player.handle == 0 {
                    info!("setting max speed...");
                    player.stats.max_speed = 0.0;
                }
            }
        }
    }

    command_queque.early_queue = command_queque.queue.clone();
    command_queque.queue = Vec::default();
}

pub struct AceConsolePlugin;

impl Plugin for AceConsolePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (push_command, receive_command)
                .chain()
                .run_if(in_state(GameState::InRollbackGame)),
        )
        .init_resource::<CommandQueue>()
        .add_systems(OnEnter(RollbackState::InRound), apply_commands);
    }
}
