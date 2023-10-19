pub mod connecting_screen;
pub mod game_over_screen;
pub mod networking_screen;
pub mod round_over_screen;
pub mod round_start_screen;
pub mod scoreboard;
pub mod seed_screen;

use bevy::prelude::*;
use bevy_ggrs::GgrsSchedule;

use crate::game_logic::{adjust_score, check_rematch, round_end_timeout};
use crate::player::check_rematch_state;
use crate::player::spawning::despawn_players;
use crate::{GameState, RollbackState};

use connecting_screen::{
    animate_connecting_screen, despawn_connecting_screen, spawn_connecting_screen,
    tick_connecting_timer,
};
use game_over_screen::{spawn_game_over_screen, update_rematch_text};
use networking_screen::{despawn_networking_screen, spawn_networking_screen};
use round_over_screen::{hide_round_over_screen, show_round_over_screen, spawn_round_over_screen};
use round_start_screen::{
    animate_round_start_screen, hide_round_start_screen, round_start_timeout,
    show_round_start_screen, spawn_round_start_screen,
};
use scoreboard::{spawn_scoreboard, update_scoreboard};
use seed_screen::spawn_seed_screen;

use self::game_over_screen::{hide_game_over_screen, show_game_over_screen, update_winner_text};

pub const MAX_SCORE: usize = 2;

pub struct AceUiPlugin;

impl Plugin for AceUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnExit(GameState::Connecting),
            (despawn_connecting_screen, spawn_seed_screen),
        )
        .add_systems(OnEnter(GameState::Matchmaking), spawn_networking_screen)
        .add_systems(
            OnExit(GameState::Matchmaking),
            (despawn_networking_screen, spawn_connecting_screen),
        )
        .add_systems(
            OnExit(RollbackState::Setup),
            (
                spawn_scoreboard,
                spawn_round_start_screen,
                spawn_round_over_screen,
                spawn_game_over_screen,
            ),
        )
        .add_systems(
            OnEnter(RollbackState::GameOver),
            (show_game_over_screen, update_winner_text),
        )
        .add_systems(OnExit(RollbackState::GameOver), hide_game_over_screen)
        .add_systems(OnExit(RollbackState::RoundEnd), hide_round_over_screen)
        .add_systems(OnEnter(RollbackState::RoundStart), show_round_start_screen)
        .add_systems(OnExit(RollbackState::RoundStart), hide_round_start_screen)
        .add_systems(
            OnEnter(RollbackState::RoundEnd),
            (show_round_over_screen.after(adjust_score),),
        )
        .add_systems(
            Update,
            animate_connecting_screen.run_if(in_state(GameState::Connecting)),
        )
        .add_systems(
            GgrsSchedule,
            (
                animate_round_start_screen
                    .run_if(in_state(RollbackState::RoundStart))
                    .after(round_start_timeout),
                hide_round_start_screen.run_if(in_state(RollbackState::InRound)),
                update_rematch_text
                    .run_if(in_state(RollbackState::GameOver))
                    .after(check_rematch_state),
                update_scoreboard
                    .run_if(
                        in_state(RollbackState::InRound)
                            .or_else(in_state(RollbackState::RoundStart))
                            .or_else(in_state(RollbackState::RoundEnd))
                            .or_else(in_state(RollbackState::GameOver)),
                    )
                    .after(check_rematch),
            )
                .chain()
                .after(apply_state_transition::<RollbackState>),
        )
        .add_systems(
            GgrsSchedule,
            (
                round_start_timeout
                    .ambiguous_with(round_end_timeout)
                    .ambiguous_with(check_rematch)
                    .ambiguous_with(despawn_players)
                    .distributive_run_if(in_state(RollbackState::RoundStart))
                    .after(apply_state_transition::<RollbackState>),
                tick_connecting_timer
                    .ambiguous_with(round_end_timeout)
                    .ambiguous_with(check_rematch)
                    .ambiguous_with(despawn_players)
                    .ambiguous_with(round_start_timeout)
                    .distributive_run_if(in_state(GameState::Connecting))
                    .after(apply_state_transition::<RollbackState>),
            ),
        );
    }
}
