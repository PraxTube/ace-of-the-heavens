use bevy::prelude::*;
use bevy_ggrs::AddRollbackCommandExtension;

use crate::debug::DebugTransform;
use crate::player::player::{Player, MAX_HEALTH, PLAYER_RADIUS};
use crate::player::shooting;

const HEALTH_BAR_OFFSET: Vec3 = Vec3::new(-30.0, -40.0, 0.0);
const HEALTH_BAR_SCALE: Vec3 = Vec3::new(60.0, 7.5, 1.0);

#[derive(Component)]
pub struct HealthBar;

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

pub fn update_health_bars(
    mut health_bars: Query<
        (&mut Transform, &Children, &mut DebugTransform),
        (With<HealthBar>, Without<Player>, Without<HealthBarFill>),
    >,
    mut health_bar_fills: Query<
        (&mut Transform, &HealthBarFill, &mut DebugTransform),
        (Without<Player>, Without<HealthBar>),
    >,
    players: Query<(&Transform, &Player, &Children), Without<HealthBar>>,
) {
    for (player_transform, player, p_children) in &players {
        for &p_child in p_children {
            let res = health_bars.get_mut(p_child);

            match res {
                Ok((mut health_bar_transform, h_children, mut health_bar_debug_transform)) => {
                    health_bar_transform.rotation = player_transform.rotation.inverse();
                    health_bar_transform.translation =
                        player_transform.rotation.mul_vec3(HEALTH_BAR_OFFSET);
                    health_bar_debug_transform.update(&health_bar_transform);

                    for &h_child in h_children {
                        let health_bar_fill = health_bar_fills.get_mut(h_child);
                        match health_bar_fill {
                            Ok(mut fill) => {
                                let x_fill = (100 * player.health / MAX_HEALTH).clamp(0, 100);
                                fill.0.scale = Vec3::new(
                                    x_fill as f32 / 100.0,
                                    fill.0.scale.y,
                                    fill.0.scale.z,
                                );
                                fill.2.update(&fill.0);
                            }
                            Err(_) => {}
                        }
                    }
                }
                Err(_) => {}
            }
        }
    }
}

pub fn spawn_health_bar(commands: &mut Commands) -> Entity {
    let transform = Transform::from_scale(HEALTH_BAR_SCALE).with_translation(Vec3::new(
        HEALTH_BAR_SCALE.x / 2.0,
        0.0,
        10.0,
    ));

    let main = commands
        .spawn((
            HealthBar,
            DebugTransform::default(),
            SpatialBundle {
                transform: Transform::from_translation(HEALTH_BAR_OFFSET),
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
    main
}
