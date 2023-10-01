use bevy::prelude::*;
use bevy_ggrs::*;

use crate::debug::DebugTransform;
use crate::input;
use crate::network::GgrsConfig;
use crate::player::player::{Player, DELTA_SPEED, DELTA_STEERING, MAX_SPEED, MIN_SPEED};

pub fn steer_players(
    inputs: Res<PlayerInputs<GgrsConfig>>,
    mut players: Query<(&mut Transform, &Player, &mut DebugTransform)>,
) {
    for (mut transform, player, mut debug_transform) in &mut players {
        let (input, _) = inputs[player.handle];

        let steer_direction = input::steer_direction(input);

        if steer_direction == 0.0 {
            continue;
        }

        let rotation = DELTA_STEERING * steer_direction;
        transform.rotate_z(rotation);
        debug_transform.update(&transform);
    }
}

pub fn accelerate_players(inputs: Res<PlayerInputs<GgrsConfig>>, mut players: Query<&mut Player>) {
    for mut player in &mut players {
        let (input, _) = inputs[player.handle];

        let accelerate_direction = input::accelerate_direction(input);

        if accelerate_direction == 0.0 {
            continue;
        }

        let acceleration = if accelerate_direction > 0.0 {
            DELTA_SPEED * 3.0
        } else {
            -DELTA_SPEED
        };

        player.current_speed += acceleration;
        player.current_speed = player.current_speed.clamp(MIN_SPEED, MAX_SPEED);
    }
}

pub fn move_players(mut players: Query<(&mut Transform, &Player, &mut DebugTransform)>) {
    for (mut transform, player, mut debug_transform) in &mut players {
        let direction = transform.local_x();
        transform.translation += direction * player.current_speed;
        debug_transform.update(&transform);
    }
}
