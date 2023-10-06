use std::f32::consts::PI;

use bevy::{prelude::*, sprite::collide_aabb::Collision};

use crate::debug::DebugTransform;
use crate::map::obstacle::{collision, Obstacle};
use crate::player::player::{Player, PLAYER_RADIUS};
use crate::player::shooting::Bullet;

use super::player::MIN_SPEED;

fn normal_vec(collision: Collision, d: Vec3) -> Option<Vec3> {
    match collision {
        Collision::Top => {
            if d.y > 0.0 {
                None
            } else {
                Some(Vec3::Y)
            }
        }
        Collision::Bottom => {
            if d.y < 0.0 {
                None
            } else {
                Some(Vec3::NEG_Y)
            }
        }
        Collision::Left => {
            if d.x < 0.0 {
                None
            } else {
                Some(Vec3::NEG_X)
            }
        }
        Collision::Right => {
            if d.x > 0.0 {
                None
            } else {
                Some(Vec3::X)
            }
        }
        _ => None,
    }
}

fn check_obstacle_collision(
    player_transform: &mut Transform,
    player: &mut Player,
    player_debug_transform: &mut DebugTransform,
    obstacle: &Obstacle,
) {
    let collision = match collision(obstacle, player_transform.translation, PLAYER_RADIUS) {
        Some(val) => val,
        None => return,
    };

    let d = player_transform.rotation.mul_vec3(Vec3::X);
    let n = match normal_vec(collision, d) {
        Some(val) => val,
        None => return,
    };

    let r = d - 2.0 * (d.dot(n)) * n;

    let angle = match collision {
        Collision::Top => r.angle_between(Vec3::X),
        Collision::Bottom => 2.0 * PI - r.angle_between(Vec3::X),
        Collision::Left => {
            if r.y < 0.0 {
                2.0 * PI - r.angle_between(Vec3::X)
            } else {
                r.angle_between(Vec3::X)
            }
        }
        Collision::Right => {
            if r.y < 0.0 {
                2.0 * PI - r.angle_between(Vec3::X)
            } else {
                r.angle_between(Vec3::X)
            }
        }
        _ => panic!("we should never be inside a collision at this point"),
    };

    player_transform.rotation = Quat::from_rotation_z(angle);
    player_transform.translation += r * 1.0;
    player_debug_transform.update(&player_transform);

    if player.health < obstacle.damage {
        player.health = 0;
    } else {
        player.health -= obstacle.damage;
        player.current_speed = MIN_SPEED;
    }
}

pub fn obstacle_collision(
    mut players: Query<(&mut Transform, &mut Player, &mut DebugTransform)>,
    obstacles: Query<&Obstacle, (Without<Player>, Without<Bullet>)>,
) {
    for (mut player_transform, mut player, mut player_debug_transform) in &mut players {
        for obstacle in &obstacles {
            check_obstacle_collision(
                &mut player_transform,
                &mut player,
                &mut player_debug_transform,
                obstacle,
            );
        }
    }
}
