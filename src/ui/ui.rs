use bevy::prelude::*;

use super::{
    game_over_screen::spawn_game_over_screen,
    round_over_screen::{hide_round_over_screen, show_round_over_screen, spawn_round_over_screen},
    scoreboard::{spawn_scoreboard, update_scoreboard},
};
use crate::{adjust_score, GameState, RollbackState};

pub const MAX_SCORE: usize = 5;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::InGame),
            (spawn_scoreboard, spawn_round_over_screen),
        )
        .add_systems(
            OnEnter(GameState::GameOver),
            (spawn_game_over_screen, hide_round_over_screen),
        )
        .add_systems(OnEnter(RollbackState::InRound), hide_round_over_screen)
        .add_systems(
            OnEnter(RollbackState::RoundEnd),
            (
                update_scoreboard.after(adjust_score),
                show_round_over_screen.after(adjust_score),
            ),
        );
    }
}
