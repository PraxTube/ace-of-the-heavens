use bevy::prelude::*;
use bevy_ggrs::{ggrs::PlayerType, *};
use bevy_matchbox::prelude::*;
use ggrs::GGRSEvent as GgrsEvent;

use crate::player::player::LocalPlayerHandle;
use crate::GameState;

#[derive(Debug)]
pub struct GgrsConfig;

impl ggrs::Config for GgrsConfig {
    type Input = u8;
    type State = u8;
    type Address = PeerId;
}

pub fn start_matchbox_socket(mut commands: Commands) {
    let room_url = "ws://192.168.178.30:3536/";
    info!("connection to matchbox server: {}", room_url);
    commands.insert_resource(MatchboxSocket::new_ggrs(room_url));
}

pub fn wait_for_players(
    mut commands: Commands,
    mut socket: ResMut<MatchboxSocket<SingleChannel>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if socket.get_channel(0).is_err() {
        return;
    }

    socket.update_peers();
    let players = socket.players();

    let num_players = 2;
    if players.len() < num_players {
        return;
    }

    info!("all peers have joined, going in-game");

    let mut session_builder = ggrs::SessionBuilder::<GgrsConfig>::new()
        .with_num_players(num_players)
        .with_desync_detection_mode(ggrs::DesyncDetection::On { interval: 10 })
        .with_input_delay(2);

    for (i, player) in players.into_iter().enumerate() {
        if player == PlayerType::Local {
            commands.insert_resource(LocalPlayerHandle(i))
        }
        session_builder = session_builder
            .add_player(player, i)
            .expect("failed to add player");
    }

    let channel = socket.take_channel(0).unwrap();

    let ggrs_session = session_builder
        .start_p2p_session(channel)
        .expect("failed to start session");

    commands.insert_resource(bevy_ggrs::Session::P2P(ggrs_session));

    next_state.set(GameState::InGame)
}

pub fn print_events_system(mut session: ResMut<Session<GgrsConfig>>) {
    match session.as_mut() {
        Session::P2P(s) => {
            for event in s.events() {
                match event {
                    GgrsEvent::Disconnected { .. } | GgrsEvent::NetworkInterrupted { .. } => {
                        warn!("{event:?}")
                    }
                    GgrsEvent::DesyncDetected { .. } => error!("{event:?}"),
                    _ => info!("{event:?}"),
                }
            }
        }
        _ => panic!("Expecting a P2P Session."),
    }
}
