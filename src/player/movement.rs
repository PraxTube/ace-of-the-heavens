use bevy::prelude::*;
use bevy_ggrs::*;

use crate::input;
use crate::network::GgrsConfig;
use crate::player::player::{Player, DELTA_SPEED, DELTA_STEERING, MAX_SPEED, MIN_SPEED};

pub fn steer_players(
    time: Res<Time>,
    inputs: Res<PlayerInputs<GgrsConfig>>,
    mut players: Query<(&mut Transform, &Player)>,
) {
    for (mut transform, player) in &mut players {
        let (input, _) = inputs[player.handle];

        let steer_direction = input::steer_direction(input);

        if steer_direction == 0.0 {
            continue;
        }

        let rotation = DELTA_STEERING * steer_direction * time.delta_seconds();
        transform.rotate_z(rotation);
    }
}

pub fn accelerate_players(
    time: Res<Time>,
    inputs: Res<PlayerInputs<GgrsConfig>>,
    mut players: Query<&mut Player>,
) {
    for mut player in &mut players {
        let (input, _) = inputs[player.handle];

        let accelerate_direction = input::accelerate_direction(input);

        if accelerate_direction == 0.0 {
            continue;
        }

        player.current_speed += DELTA_SPEED * accelerate_direction * time.delta_seconds();
        player.current_speed = player.current_speed.clamp(MIN_SPEED, MAX_SPEED);
    }
}

pub fn move_players(time: Res<Time>, mut players: Query<(&mut Transform, &Player)>) {
    for (mut transform, player) in &mut players {
        let direction = transform.local_x();
        transform.translation += direction * player.current_speed * time.delta_seconds();
    }
}
