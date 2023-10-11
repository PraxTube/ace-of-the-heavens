use std::{f32::consts::PI, time::Duration};

use bevy::prelude::*;
use bevy_ggrs::PlayerInputs;

use crate::{input::dodge, network::GgrsConfig};

use super::player::Player;

const DODGE_COOLDOWN: f32 = 5.0;
const DODGE_TIME: f32 = 0.5;

#[derive(Component)]
pub struct DodgeTimer(Timer);

impl Default for DodgeTimer {
    fn default() -> DodgeTimer {
        let mut timer = Timer::from_seconds(DODGE_COOLDOWN, TimerMode::Once);
        timer.set_elapsed(Duration::from_secs_f32(DODGE_COOLDOWN));
        DodgeTimer(timer)
    }
}

pub fn initiate_dodge(
    mut timers: Query<(&mut DodgeTimer, &mut Player)>,
    inputs: Res<PlayerInputs<GgrsConfig>>,
) {
    for (mut timer, mut player) in &mut timers {
        let (input, _) = inputs[player.handle];
        if timer.0.finished() && dodge(input) {
            player.dodging = true;
            timer.0.reset();
        }
    }
}

pub fn animate_dodge(mut players: Query<(&mut Transform, &mut Player, &DodgeTimer)>) {
    for (mut transform, mut player, timer) in &mut players {
        if timer.0.elapsed_secs() > DODGE_TIME {
            player.dodging = false;
            continue;
        }

        transform.rotate_local_x(2.0 * PI / DODGE_TIME / 60.0);
    }
}

pub fn tick_dodge_timer(mut timers: Query<&mut DodgeTimer, With<Player>>) {
    for mut timer in &mut timers {
        if timer.0.finished() {
            continue;
        }

        timer.0.tick(Duration::from_secs_f32(1.0 / 60.0));
    }
}
