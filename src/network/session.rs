use bevy::prelude::*;
use bevy_ggrs::{ggrs::PlayerType, *};
use bevy_matchbox::matchbox_socket::WebRtcSocket;

use super::ggrs_config::PLAYER_COUNT;
use super::socket::AceSocket;
use super::GgrsConfig;
use crate::game_logic::{Seed, Seeds};
use crate::player::player::LocalPlayerHandle;
use crate::GameState;

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
    mut next_state: ResMut<NextState<GameState>>,
    seed: Res<Seeds>,
) {
    // We might need this, we would probably need to do something like
    // socket.innter_mut or socket.inner and then get_channel
    // if socket.get_channel(0).is_err() {
    //     return;
    // }

    let _new_peers = socket.inner_mut().update_peers();

    let players = socket.players();

    if players.len() < PLAYER_COUNT {
        return;
    }
    if players.len() > PLAYER_COUNT {
        error!("You are trying to join an already full game! Exiting to main menu.");
        return;
    }

    info!("All peers have joined, going in-game");

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
    next_state.set(GameState::Connecting);
}

pub fn wait_for_seed(mut seeds: ResMut<Seeds>, mut socket: ResMut<AceSocket>) {
    let received_seeds = socket.receive_tcp_seed();

    if received_seeds.len() == 0 {
        return;
    }

    info!("received seeds");

    for seed in received_seeds {
        seeds.0.push(Seed {
            handle: Some(seed.0),
            seed: seed.1,
        });
    }
}
