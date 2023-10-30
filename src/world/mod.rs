mod clear;
mod round;
mod seed;

pub use round::{
    adjust_score, check_rematch, round_end_timeout, Rematch, RoundEndTimer, RoundStats, Score,
    MAX_SCORE,
};
pub use seed::{determine_seed, Seed, SeedHandle, Seeds};

use bevy::prelude::*;

pub struct GameLogicPlugin;

impl Plugin for GameLogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            seed::WorldSeedPlugin,
            clear::WorldClearPlugin,
            round::WorldRoundPlugin,
        ));
    }
}
