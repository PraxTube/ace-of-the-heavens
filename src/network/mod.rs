pub mod ggrs_config;
pub mod session;
pub mod socket;

pub use ggrs_config::GgrsConfig;

use bevy::prelude::*;

use crate::GameState;
use session::{check_ready_state, start_matchbox_socket, wait_for_players, wait_for_seed, Ready};
use socket::AceSocket;

pub struct AceNetworkPlugin;

impl Plugin for AceNetworkPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                wait_for_players.run_if(in_state(GameState::Matchmaking)),
                wait_for_seed
                    .run_if(in_state(GameState::Matchmaking))
                    .run_if(resource_exists::<AceSocket>()),
                check_ready_state.run_if(in_state(GameState::Matchmaking)),
            ),
        )
        .init_resource::<Ready>()
        .add_systems(OnEnter(GameState::Matchmaking), start_matchbox_socket);
    }
}
