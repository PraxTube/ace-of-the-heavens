pub mod round_start_screen;

mod game_over_screen;
mod main_menu_screen;
mod matchmaking_screen;
mod round_over_screen;
mod scoreboard;
mod seed_screen;
mod session_stats_screen;

use bevy::prelude::*;

pub struct AceUiPlugin;

impl Plugin for AceUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            main_menu_screen::MainMenuUiPlugin,
            matchmaking_screen::MatchmakingUiPlugin,
            scoreboard::ScoreboardUiPlugin,
            session_stats_screen::SessionStatsPlugin,
            seed_screen::SeedUiPlugin,
            round_start_screen::RoundStartUiPlugin,
            round_over_screen::RoundOverUiPlugin,
            game_over_screen::GameOverUiPlugin,
        ));
    }
}
