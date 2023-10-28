pub mod game_over_screen;
pub mod help_menu_screen;
pub mod main_menu_screen;
pub mod networking_screen;
pub mod round_over_screen;
pub mod round_start_screen;
pub mod scoreboard;
pub mod seed_screen;
pub mod session_stats_screen;

use bevy::prelude::*;
use bevy_ggrs::GgrsSchedule;

use crate::game_logic::{adjust_score, check_rematch, round_end_timeout};
use crate::player::check_rematch_state;
use crate::player::spawning::despawn_players;
use crate::{GameState, RollbackState};

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
use self::help_menu_screen::{
    despawn_help_menu_screen, return_to_main, scroll_help_screen, spawn_help_menu_screen,
};
use self::main_menu_screen::{
    despawn_main_menu_screen, help_menu, play_game, spawn_main_menu_screen,
};
use self::session_stats_screen::{spawn_stats_text, toggle_stats_visibility, update_stats_text};

pub const MAX_SCORE: usize = 5;

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum MainMenuState {
    #[default]
    MainMenu,
    HelpMenu,
}

pub struct AceUiPlugin;

impl Plugin for AceUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnExit(GameState::Matchmaking),
            (despawn_networking_screen, spawn_seed_screen),
        )
        .add_state::<MainMenuState>()
        .add_systems(OnEnter(GameState::InRollbackGame), spawn_stats_text)
        .add_systems(OnEnter(GameState::MainMenu), spawn_main_menu_screen)
        .add_systems(OnExit(GameState::MainMenu), despawn_main_menu_screen)
        .add_systems(
            OnEnter(MainMenuState::HelpMenu),
            (spawn_help_menu_screen, despawn_main_menu_screen),
        )
        .add_systems(
            OnExit(MainMenuState::HelpMenu),
            (despawn_help_menu_screen, spawn_main_menu_screen),
        )
        .add_systems(OnEnter(GameState::Matchmaking), spawn_networking_screen)
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
            (
                play_game.run_if(
                    in_state(GameState::MainMenu).and_then(in_state(MainMenuState::MainMenu)),
                ),
                help_menu.run_if(
                    in_state(GameState::MainMenu).and_then(in_state(MainMenuState::MainMenu)),
                ),
                return_to_main.run_if(
                    in_state(GameState::MainMenu).and_then(in_state(MainMenuState::HelpMenu)),
                ),
                scroll_help_screen.run_if(
                    in_state(GameState::MainMenu).and_then(in_state(MainMenuState::HelpMenu)),
                ),
                update_stats_text.run_if(in_state(GameState::InRollbackGame)),
                toggle_stats_visibility.run_if(in_state(GameState::InRollbackGame)),
            ),
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
            round_start_timeout
                .ambiguous_with(round_end_timeout)
                .ambiguous_with(check_rematch)
                .ambiguous_with(despawn_players)
                .distributive_run_if(in_state(RollbackState::RoundStart))
                .after(apply_state_transition::<RollbackState>),
        );
    }
}
