pub mod round_start_screen;

mod game_over_screen;
mod help_menu_screen;
mod main_menu_screen;
mod matchmaking_screen;
mod round_over_screen;
mod scoreboard;
mod seed_screen;
mod session_stats_screen;

use bevy::prelude::*;

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
        app.add_plugins((
            main_menu_screen::MainMenuUiPlugin,
            help_menu_screen::HelpMenuUiPlugin,
            matchmaking_screen::MatchmakingUiPlugin,
            scoreboard::ScoreboardUiPlugin,
            session_stats_screen::SessionStatsPlugin,
            seed_screen::SeedUiPlugin,
            round_start_screen::RoundStartUiPlugin,
            round_over_screen::RoundOverUiPlugin,
            game_over_screen::GameOverUiPlugin,
        ))
        .add_state::<MainMenuState>();
    }
}
