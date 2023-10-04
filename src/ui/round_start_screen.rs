use bevy::prelude::*;

use crate::player::player::{P1_COLOR, P2_COLOR};
use crate::RollbackState;

#[derive(Resource, Reflect, Deref, DerefMut)]
#[reflect(Resource)]
pub struct RoundStartTimer(Timer);

impl Default for RoundStartTimer {
    fn default() -> Self {
        RoundStartTimer(Timer::from_seconds(1.0, TimerMode::Repeating))
    }
}

pub fn spawn_round_start_screen(mut commands: Commands, asset_server: Res<AssetServer>) {}

pub fn round_start_timeout(
    mut timer: ResMut<RoundStartTimer>,
    mut next_state: ResMut<NextState<RollbackState>>,
) {
    timer.tick(std::time::Duration::from_secs_f32(1.0 / 60.0));

    info!("{}", timer.duration().as_secs_f32());

    if timer.just_finished() {
        next_state.set(RollbackState::InRound);
    }
}
