use bevy::prelude::*;
use bevy_ggrs::{ggrs::PlayerType, *};
use bevy_matchbox::matchbox_socket::WebRtcSocket;

use super::ggrs_config::PLAYER_COUNT;
use super::socket::AceSocket;
use super::GgrsConfig;
use crate::game_logic::{SeedHandle, Seeds};
use crate::player::LocalPlayerHandle;
use crate::{GameState, RollbackState};

#[derive(Resource)]
pub struct Ready {
    connection_ready: bool,
    local_ready: bool,
    remote_ready: bool,
}

impl Default for Ready {
    fn default() -> Self {
        Self {
            connection_ready: false,
            local_ready: false,
            remote_ready: false,
        }
    }
}

pub fn start_matchbox_socket(mut commands: Commands) {
    let room_url = "ws://192.168.178.98:3536/";
    info!("connection to matchbox server: {}", room_url);
    commands.insert_resource(AceSocket::from(
        WebRtcSocket::builder(room_url)
            .add_ggrs_channel()
            .add_reliable_channel()
            .build(),
    ));
}

/// Initialize the multiplayer session.
/// Having input systems in GGRS schedule will not execute them until a session is initialized.
/// Will wait until all players have joined.
pub fn wait_for_players(
    mut commands: Commands,
    mut socket: ResMut<AceSocket>,
    mut ready: ResMut<Ready>,
    seed: Res<Seeds>,
) {
    if socket.inner_mut().get_channel(0).is_err() {
        return;
    }

    let _new_peers = socket.inner_mut().update_peers();

    let players = socket.players();

    if players.len() < PLAYER_COUNT {
        return;
    }
    if players.len() > PLAYER_COUNT {
        error!("You are trying to join an already full game! Exiting to main menu.");
        return;
    }

    info!("all peers have joined!");

    let mut session_builder = GgrsConfig::new_builder();

    for (i, player) in players.into_iter().enumerate() {
        session_builder = session_builder
            .add_player(player, i)
            .expect("failed to add player");

        match player {
            PlayerType::Remote(peer_id) => {
                socket.send_tcp_seed(peer_id, seed.0[0].seed);
            }
            PlayerType::Local => {
                commands.insert_resource(LocalPlayerHandle(i));
            }
            PlayerType::Spectator(_) => {}
        };
    }

    // move the channel out of the socket (required because GGRS takes ownership of it)
    let channel = socket
        .inner_mut()
        .take_channel(AceSocket::GGRS_CHANNEL)
        .unwrap();

    // start the GGRS session
    let ggrs_session = session_builder
        .start_p2p_session(channel)
        .expect("failed to start session");

    commands.insert_resource(Session::P2P(ggrs_session));
    ready.connection_ready = true;
}

pub fn wait_for_seed(
    mut seeds: ResMut<Seeds>,
    mut socket: ResMut<AceSocket>,
    mut ready: ResMut<Ready>,
) {
    if !ready.connection_ready {
        return;
    }

    let received_seeds = socket.receive_tcp_seed();

    if received_seeds.len() == 0 {
        return;
    }

    for seed in received_seeds {
        // Ready signal from peer
        if seed.1 == 0 {
            ready.remote_ready = true;
            info!("peer is ready, received 0 seed");
            continue;
        }

        // Normal seed
        seeds.0.push(SeedHandle {
            handle: Some(seed.0),
            seed: seed.1,
        });

        ready.local_ready = true;
        // Send the 0 seed as a confirmation that we are ready.
        // The 0 seed should never be possible as a normal seed.
        for player in socket.players() {
            match player {
                PlayerType::Remote(peer_id) => {
                    socket.send_tcp_seed(peer_id, 0);
                }
                _ => {}
            };
        }
        info!("we are ready, received peer seed and sent 0 seed");
    }
}

pub fn check_ready_state(
    ready: Res<Ready>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_rollback_state: ResMut<NextState<RollbackState>>,
) {
    if ready.local_ready && ready.remote_ready {
        if !ready.connection_ready {
            // sanity check, should never trigger
            panic!("conneciton is not established but we are ready to play?")
        }
        next_game_state.set(GameState::InRollbackGame);
        next_rollback_state.set(RollbackState::RoundStart);
    }
}
