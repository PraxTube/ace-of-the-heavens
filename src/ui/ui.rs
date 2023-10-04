use bevy::prelude::*;
use bevy_ggrs::GgrsSchedule;

use super::{
    game_over_screen::spawn_game_over_screen,
    round_over_screen::{hide_round_over_screen, show_round_over_screen, spawn_round_over_screen},
    round_start_screen::{round_start_timeout, spawn_round_start_screen},
    scoreboard::{spawn_scoreboard, update_scoreboard},
};
use crate::player::player::destroy_players;
use crate::{adjust_score, round_end_timeout, GameState, RollbackState};

pub const MAX_SCORE: usize = 5;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::InGame),
            (
                spawn_scoreboard,
                spawn_round_over_screen,
                spawn_round_start_screen,
            ),
        )
        .add_systems(
            OnEnter(GameState::GameOver),
            (spawn_game_over_screen, hide_round_over_screen),
        )
        .add_systems(OnEnter(RollbackState::RoundStart), hide_round_over_screen)
        .add_systems(
            OnEnter(RollbackState::RoundEnd),
            (
                update_scoreboard.after(adjust_score),
                show_round_over_screen.after(adjust_score),
            ),
        )
        .add_systems(
            GgrsSchedule,
            round_start_timeout
                .ambiguous_with(round_end_timeout)
                .ambiguous_with(destroy_players)
                .distributive_run_if(in_state(RollbackState::RoundStart))
                .after(apply_state_transition::<RollbackState>),
        );
    }
}
