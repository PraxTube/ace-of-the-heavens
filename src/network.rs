use bevy::prelude::*;
use bevy_ggrs::*;
use bevy_matchbox::prelude::*;
use ggrs::{PlayerType, SessionBuilder};

use crate::GameState;

#[derive(Debug)]
pub struct GgrsConfig;

impl ggrs::Config for GgrsConfig {
    type Input = u8;
    type State = u8;
    type Address = PeerId;
}

pub fn wait_for_players(mut commands: Commands, mut next_state: ResMut<NextState<GameState>>) {
    let num_players = 2;
    let mut sess_build = SessionBuilder::<GgrsConfig>::new()
        .with_num_players(num_players)
        .with_check_distance(7)
        .with_input_delay(2);

    for i in 0..num_players {
        sess_build = sess_build
            .add_player(PlayerType::Local, i)
            .expect("failed to add player");
    }

    // start the GGRS session
    let sess = sess_build
        .start_synctest_session()
        .expect("failed to start session");

    commands.insert_resource(bevy_ggrs::Session::SyncTest(sess));

    next_state.set(GameState::InGame)
}
