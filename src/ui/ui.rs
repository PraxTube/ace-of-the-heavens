use bevy::prelude::*;

use super::{round_over_screen, scoreboard};

pub const MAX_SCORE: usize = 5;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    scoreboard::spawn_scoreboard(&mut commands, &asset_server);
    round_over_screen::spawn_round_over_screen(&mut commands, &asset_server);
}
