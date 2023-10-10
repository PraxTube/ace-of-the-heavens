use bevy::prelude::*;

use super::session::{start_matchbox_socket, wait_for_players, wait_for_seed};
use crate::GameState;

pub struct AceNetworkPlugin;

impl Plugin for AceNetworkPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                wait_for_players.run_if(in_state(GameState::Matchmaking)),
                wait_for_seed.run_if(in_state(GameState::Connecting)),
            ),
        )
        .add_systems(OnEnter(GameState::Matchmaking), start_matchbox_socket);
    }
}
