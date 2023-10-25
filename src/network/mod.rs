pub mod ggrs_config;
pub mod session;
pub mod session_event;
pub mod session_stats;
pub mod socket;

use bevy_ggrs::GgrsSchedule;
pub use ggrs_config::GgrsConfig;

use bevy::prelude::*;

use crate::{
    game_logic::{check_rematch, round_end_timeout},
    player::spawning::despawn_players,
    ui::round_start_screen::round_start_timeout,
    GameState, RollbackState,
};
use session::{check_ready_state, start_matchbox_socket, wait_for_players, wait_for_seed, Ready};
use session_event::{
    change_game_state, change_rollback_state, handle_session_events, SessionEvent,
};
use socket::AceSocket;

use self::session_stats::{update_session_stats, SessionStats};

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
                handle_session_events.run_if(in_state(GameState::InRollbackGame)),
                update_session_stats
                    .run_if(in_state(GameState::InRollbackGame))
                    .after(handle_session_events),
                change_game_state.run_if(in_state(GameState::InRollbackGame)),
            ),
        )
        .init_resource::<Ready>()
        .init_resource::<SessionStats>()
        .add_event::<SessionEvent>()
        .add_systems(OnEnter(GameState::Matchmaking), start_matchbox_socket)
        .add_systems(
            GgrsSchedule,
            change_rollback_state
                .run_if(not(in_state(RollbackState::Setup)))
                .after(despawn_players)
                .after(check_rematch)
                .after(round_start_timeout)
                .after(round_end_timeout)
                .after(apply_state_transition::<RollbackState>),
        );
    }
}
