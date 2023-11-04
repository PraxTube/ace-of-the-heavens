use bevy::prelude::*;
use bevy_ggrs::*;

use crate::debug::DebugTransform;
use crate::input;
use crate::network::GgrsConfig;
use crate::player::{Player, DELTA_SPEED, DELTA_STEERING, MIN_SPEED};

#[derive(Event)]
pub struct ReachedMaxSpeed {
    pub position: Vec3,
    pub direction: Vec3,
}

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

        let move_ratio =
            1.0 - (player.current_speed - MIN_SPEED) / (player.stats.max_speed - MIN_SPEED);
        let move_ratio = (1.0 + move_ratio * 0.22474487).powi(2);
        let rotation = DELTA_STEERING * steer_direction * move_ratio;
        transform.rotate_z(rotation);
        debug_transform.update(&transform);
    }
}

pub fn accelerate_players(
    inputs: Res<PlayerInputs<GgrsConfig>>,
    mut players: Query<(&Transform, &mut Player)>,
    mut ev_reached_max_speed: EventWriter<ReachedMaxSpeed>,
) {
    for (transform, mut player) in &mut players {
        let (input, _) = inputs[player.handle];

        let accelerate_direction = input::accelerate_direction(input);

        if accelerate_direction == 0.0 {
            continue;
        }

        let acceleration = if accelerate_direction > 0.0 {
            DELTA_SPEED
        } else {
            -DELTA_SPEED
        };

        let sub_sonic = player.current_speed != player.stats.max_speed;

        player.current_speed += acceleration;
        player.current_speed = player
            .current_speed
            .clamp(MIN_SPEED, player.stats.max_speed);

        // We just reached max speed this frame, send event
        if sub_sonic && player.current_speed == player.stats.max_speed {
            ev_reached_max_speed.send(ReachedMaxSpeed {
                position: transform.translation,
                direction: transform.rotation.mul_vec3(Vec3::X),
            });
        }
    }
}

pub fn move_players(mut players: Query<(&mut Transform, &Player, &mut DebugTransform)>) {
    for (mut transform, player, mut debug_transform) in &mut players {
        let direction = transform.local_x();
        transform.translation += direction * player.current_speed;
        debug_transform.update(&transform);
    }
}
