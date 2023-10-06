use std::f32::consts::PI;

use bevy::{prelude::*, sprite::collide_aabb::Collision};
use bevy_ggrs::AddRollbackCommandExtension;

use crate::debug::DebugTransform;
use crate::map::obstacle::{collision, Obstacle};
use crate::player::player::{Player, MAX_HEALTH, PLAYER_RADIUS};
use crate::player::shooting;
use crate::player::shooting::Bullet;

use super::player::MIN_SPEED;

const HEALTH_BAR_OFFSET: Vec3 = Vec3::new(-30.0, -40.0, 0.0);
const HEALTH_BAR_SCALE: Vec3 = Vec3::new(60.0, 7.5, 1.0);

#[derive(Component)]
pub struct HealthBar {
    pub handle: usize,
}

#[derive(Component)]
pub struct HealthBarFill;

pub fn damage_players(
    mut players: Query<(&Transform, &mut Player), Without<shooting::Bullet>>,
    mut bullets: Query<(&Transform, &mut shooting::Bullet)>,
) {
    for (player_transform, mut player) in &mut players {
        for (bullet_tranform, mut bullet) in &mut bullets {
            if bullet.handle == player.handle {
                continue;
            }
            if bullet.disabled {
                continue;
            }
            // This can happen when multiple bullets hit the player at the same time
            if player.health == 0 {
                continue;
            }

            let distance = Vec2::distance_squared(
                player_transform.translation.truncate(),
                bullet_tranform.translation.truncate(),
            );
            if distance
                < PLAYER_RADIUS * PLAYER_RADIUS + shooting::BULLET_RADIUS * shooting::BULLET_RADIUS
            {
                if player.health < bullet.damage {
                    player.health = 0;
                } else {
                    player.health -= bullet.damage;
                }
                bullet.disabled = true;
            }
        }
    }
}

pub fn obstacle_collision(
    mut players: Query<(&mut Transform, &mut Player, &mut DebugTransform)>,
    obstacles: Query<&Obstacle, (Without<Player>, Without<Bullet>)>,
) {
    for (mut player_transform, mut player, mut player_debug_transform) in &mut players {
        for obstacle in &obstacles {
            let collision = collision(obstacle, player_transform.translation, PLAYER_RADIUS);
            if collision.is_none() {
                continue;
            }

            let collision = collision.unwrap();
            info!("{:?}", collision);

            let d = player_transform.rotation.mul_vec3(Vec3::X);
            let n = match collision {
                Collision::Top => {
                    if d.y > 0.0 {
                        continue;
                    }
                    Vec3::Y
                }
                Collision::Bottom => {
                    if d.y < 0.0 {
                        continue;
                    }
                    Vec3::NEG_Y
                }
                Collision::Left => {
                    if d.x < 0.0 {
                        continue;
                    }
                    Vec3::NEG_X
                }
                Collision::Right => {
                    if d.x > 0.0 {
                        continue;
                    }
                    Vec3::X
                }
                _ => continue,
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

            info!("{}", angle);
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
    }
}

pub fn update_health_bars(
    mut health_bars: Query<
        (
            &mut Transform,
            &HealthBar,
            &Children,
            &mut Visibility,
            &mut DebugTransform,
        ),
        (Without<Player>, Without<HealthBarFill>),
    >,
    mut health_bar_fills: Query<
        (&mut Transform, &HealthBarFill, &mut DebugTransform),
        (Without<Player>, Without<HealthBar>),
    >,
    players: Query<(&Transform, &Player), Without<HealthBar>>,
) {
    for (player_transform, player) in &players {
        for (
            mut health_bar_transform,
            health_bar,
            children,
            mut health_bar_visibility,
            mut health_bar_debug_transform,
        ) in &mut health_bars
        {
            if player.handle != health_bar.handle {
                continue;
            }

            health_bar_transform.translation = player_transform.translation + HEALTH_BAR_OFFSET;
            health_bar_debug_transform.update(&health_bar_transform);

            for &child in children {
                let health_bar_fill = health_bar_fills.get_mut(child);
                match health_bar_fill {
                    Ok(mut fill) => {
                        let x_fill = (100 * player.health / MAX_HEALTH).clamp(0, 100);
                        fill.0.scale =
                            Vec3::new(x_fill as f32 / 100.0, fill.0.scale.y, fill.0.scale.z);
                        if x_fill == 0 {
                            *health_bar_visibility = Visibility::Hidden;
                        }
                        fill.2.update(&fill.0);
                    }
                    Err(_) => {}
                }
            }
        }
    }
}

pub fn spawn_health_bar(commands: &mut Commands, handle: usize, spawn_position: Vec3) {
    let transform = Transform::from_scale(HEALTH_BAR_SCALE).with_translation(Vec3::new(
        HEALTH_BAR_SCALE.x / 2.0,
        0.0,
        10.0,
    ));

    let main = commands
        .spawn((
            HealthBar { handle },
            DebugTransform::default(),
            SpatialBundle {
                transform: Transform::from_translation(spawn_position + HEALTH_BAR_OFFSET),
                ..default()
            },
        ))
        .add_rollback()
        .id();
    let background = commands
        .spawn((
            DebugTransform::new(&transform),
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.2, 0.2, 0.2),
                    custom_size: Some(Vec2::new(1.0, 1.0)),
                    ..default()
                },
                transform,
                ..default()
            },
        ))
        .add_rollback()
        .id();
    let outer = commands
        .spawn((
            HealthBarFill,
            DebugTransform::default(),
            SpatialBundle::default(),
        ))
        .add_rollback()
        .id();

    let transform = Transform::from_scale(HEALTH_BAR_SCALE).with_translation(Vec3::new(
        HEALTH_BAR_SCALE.x / 2.0,
        0.0,
        20.0,
    ));
    let inner = commands
        .spawn((
            DebugTransform::new(&transform),
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.8, 0.0, 0.0),
                    custom_size: Some(Vec2::new(1.0, 1.0)),
                    ..default()
                },
                transform,
                ..default()
            },
        ))
        .add_rollback()
        .id();
    commands.entity(outer).push_children(&[inner]);
    commands.entity(main).push_children(&[outer, background]);
}
