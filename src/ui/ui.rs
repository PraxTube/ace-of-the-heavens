use bevy::prelude::*;
use bevy_ggrs::GgrsSchedule;

use super::{
    connecting_screen::{
        animate_connecting_screen, despawn_connecting_screen, spawn_connecting_screen,
        tick_connecting_timer,
    },
    game_over_screen::{despawn_game_over_screen, spawn_game_over_screen, update_rematch_text},
    networking_screen::{despawn_networking_screen, spawn_networking_screen},
    round_over_screen::{hide_round_over_screen, show_round_over_screen, spawn_round_over_screen},
    round_start_screen::{
        animate_round_start_screen, hide_round_start_screen, round_start_timeout,
        show_round_start_screen, spawn_round_start_screen,
    },
    scoreboard::{spawn_scoreboard, update_scoreboard},
};
use crate::game_logic::{adjust_score, initiate_rematch, round_end_timeout};
use crate::player::spawning::despawn_players;
use crate::{GameState, RollbackState};

pub const MAX_SCORE: usize = 2;

pub struct AceUiPlugin;

impl Plugin for AceUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnExit(GameState::Connecting),
            (
                despawn_connecting_screen,
                spawn_scoreboard,
                spawn_round_over_screen,
                spawn_round_start_screen,
            ),
        )
        .add_systems(
            OnExit(GameState::Matchmaking),
            (despawn_networking_screen, spawn_connecting_screen),
        )
        .add_systems(OnEnter(GameState::Matchmaking), spawn_networking_screen)
        .add_systems(OnEnter(GameState::GameOver), spawn_game_over_screen)
        .add_systems(
            OnExit(GameState::GameOver),
            (despawn_game_over_screen, update_scoreboard),
        )
        .add_systems(OnExit(RollbackState::RoundEnd), hide_round_over_screen)
        .add_systems(OnEnter(RollbackState::RoundStart), show_round_start_screen)
        .add_systems(OnExit(RollbackState::RoundStart), hide_round_start_screen)
        .add_systems(
            OnEnter(RollbackState::RoundEnd),
            (
                update_scoreboard.after(adjust_score),
                show_round_over_screen.after(adjust_score),
            ),
        )
        .add_systems(
            Update,
            (
                animate_round_start_screen
                    .run_if(in_state(RollbackState::RoundStart))
                    .run_if(in_state(GameState::InGame)),
                animate_connecting_screen.run_if(in_state(GameState::Connecting)),
                hide_round_start_screen.run_if(in_state(RollbackState::InRound)),
                update_rematch_text.run_if(in_state(RollbackState::GameOver)),
            ),
        )
        .add_systems(
            GgrsSchedule,
            (
                round_start_timeout
                    .ambiguous_with(round_end_timeout)
                    .ambiguous_with(initiate_rematch)
                    .ambiguous_with(despawn_players)
                    .distributive_run_if(in_state(RollbackState::RoundStart))
                    .after(apply_state_transition::<RollbackState>),
                tick_connecting_timer
                    .ambiguous_with(round_end_timeout)
                    .ambiguous_with(initiate_rematch)
                    .ambiguous_with(despawn_players)
                    .ambiguous_with(round_start_timeout)
                    .distributive_run_if(in_state(GameState::Connecting))
                    .after(apply_state_transition::<RollbackState>),
            ),
        );
    }
}
