use bevy::prelude::*;
use bevy_ggrs::{ggrs::PlayerType, *};
use bevy_matchbox::matchbox_socket::WebRtcSocket;

use super::ggrs_config::PLAYER_COUNT;
use super::socket::AceSocket;
use super::GgrsConfig;
use crate::assets::TurnCredentials;
use crate::network::ggrs_config::get_rtc_ice_server_config;
use crate::player::LocalPlayerHandle;
use crate::world::{SeedHandle, Seeds};
use crate::{GameAssets, GameState, RollbackState};

#[derive(Resource, Default)]
pub struct Ready {
    connection_ready: bool,
    local_ready: bool,
    remote_ready: bool,
}

pub fn start_matchbox_socket(
    mut commands: Commands,
    credentials: Res<Assets<TurnCredentials>>,
    assets: Res<GameAssets>,
) {
    let room_url = format!("wss://rancic.org/matchmaking?next={}", PLAYER_COUNT);
    info!("connection to matchbox server: {}", room_url);

    let credentials = credentials.get(&assets.turn_credentials);
    commands.insert_resource(AceSocket::from(
        WebRtcSocket::builder(room_url)
            .ice_server(get_rtc_ice_server_config(credentials))
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
                socket.send_tcp_message(peer_id, &seed.0[0].seed.to_string());
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

    let received_seeds = socket.receive_tcp_message();

    if received_seeds.is_empty() {
        return;
    }

    for seed in received_seeds {
        // Ready signal from peer
        if seed.1 == "ready" {
            ready.remote_ready = true;
            info!("peer is ready, received ready message");
            continue;
        }

        // Normal seed
        seeds.0.push(SeedHandle {
            handle: Some(seed.0),
            seed: seed.1.parse::<u32>().expect("received seed is not a u32"),
        });

        ready.local_ready = true;
        // Send the 0 seed as a confirmation that we are ready.
        // The 0 seed should never be possible as a normal seed.
        for player in socket.players() {
            if let PlayerType::Remote(peer_id) = player {
                socket.send_tcp_message(peer_id, "ready");
            };
        }
        info!("we are ready, received peer seed and sent ready message");
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
