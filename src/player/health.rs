use bevy::prelude::*;
use bevy_ggrs::AddRollbackCommandExtension;

use crate::debug::DebugTransform;
use crate::player::player::{Player, MAX_HEALTH, PLAYER_RADIUS};
use crate::player::shooting::bullet::{Bullet, BULLET_RADIUS};

use super::spawning::player_spawn_transform;

const HEALTH_BAR_OFFSET: Vec3 = Vec3::new(-30.0, -40.0, 0.0);
const HEALTH_BAR_SCALE: Vec3 = Vec3::new(60.0, 7.5, 1.0);

#[derive(Component)]
pub struct HealthBar {
    pub handle: usize,
}

#[derive(Component)]
pub struct HealthBarFill;

#[derive(Event)]
pub struct PlayerTookDamage(pub Transform, pub usize);

pub fn damage_players(
    mut players: Query<(&Transform, &mut Player), Without<Bullet>>,
    mut bullets: Query<(&Transform, &mut Bullet)>,
    mut ev_player_took_damge: EventWriter<PlayerTookDamage>,
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
            if player.dodging {
                continue;
            }

            let distance = Vec2::distance_squared(
                player_transform.translation.truncate(),
                bullet_tranform.translation.truncate(),
            );
            if distance < PLAYER_RADIUS.powi(2) + BULLET_RADIUS.powi(2) {
                if player.health < bullet.damage {
                    player.health = 0;
                } else {
                    player.health -= bullet.damage;
                }
                ev_player_took_damge.send(PlayerTookDamage(*player_transform, player.handle));
                bullet.disabled = true;
            }
        }
    }
}

pub fn move_health_bars(
    mut health_bars: Query<
        (&HealthBar, &mut Transform, &mut DebugTransform),
        (Without<Player>, Without<HealthBarFill>),
    >,
    players: Query<(&Transform, &Player), Without<HealthBar>>,
) {
    for (player_transform, player) in &players {
        for (health_bar, mut health_bar_transform, mut health_bar_debug_transform) in
            &mut health_bars
        {
            if player.handle != health_bar.handle {
                continue;
            }

            health_bar_transform.translation = player_transform.translation + HEALTH_BAR_OFFSET;
            health_bar_debug_transform.update(&health_bar_transform);
        }
    }
}

fn fill_health_bar(
    health_bar_fills: &mut Query<
        (&mut Transform, &HealthBarFill, &mut DebugTransform),
        (Without<Player>, Without<HealthBar>),
    >,
    children: &Children,
    health_bar_visibility: &mut Visibility,
    player_health: u32,
) {
    for &child in children {
        let health_bar_fill = health_bar_fills.get_mut(child);
        match health_bar_fill {
            Ok(mut fill) => {
                let x_fill = (100 * player_health / MAX_HEALTH).clamp(0, 100);
                fill.0.scale = Vec3::new(x_fill as f32 / 100.0, fill.0.scale.y, fill.0.scale.z);
                if x_fill == 0 {
                    *health_bar_visibility = Visibility::Hidden;
                }
                fill.2.update(&fill.0);
            }
            Err(_) => {}
        }
    }
}

pub fn fill_health_bars(
    mut health_bars: Query<
        (&HealthBar, &Children, &mut Visibility),
        (Without<Player>, Without<HealthBarFill>),
    >,
    mut health_bar_fills: Query<
        (&mut Transform, &HealthBarFill, &mut DebugTransform),
        (Without<Player>, Without<HealthBar>),
    >,
    players: Query<&Player, Without<HealthBar>>,
) {
    for player in &players {
        for (health_bar, children, mut health_bar_visibility) in &mut health_bars {
            if player.handle != health_bar.handle {
                continue;
            }

            fill_health_bar(
                &mut health_bar_fills,
                children,
                &mut health_bar_visibility,
                player.health,
            );
        }
    }
}

fn spawn_container(commands: &mut Commands, spawn_position: Vec3, handle: usize) -> Entity {
    commands
        .spawn((
            HealthBar { handle },
            DebugTransform::default(),
            SpatialBundle {
                transform: Transform::from_translation(spawn_position + HEALTH_BAR_OFFSET),
                ..default()
            },
        ))
        .add_rollback()
        .id()
}

fn spawn_background(commands: &mut Commands) -> Entity {
    let transform = Transform::from_scale(HEALTH_BAR_SCALE).with_translation(Vec3::new(
        HEALTH_BAR_SCALE.x / 2.0,
        0.0,
        10.0,
    ));
    commands
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
        .id()
}

fn spawn_fill_container(commands: &mut Commands) -> Entity {
    commands
        .spawn((
            HealthBarFill,
            DebugTransform::default(),
            SpatialBundle::default(),
        ))
        .add_rollback()
        .id()
}

fn spawn_fill(commands: &mut Commands) -> Entity {
    let transform = Transform::from_scale(HEALTH_BAR_SCALE).with_translation(Vec3::new(
        HEALTH_BAR_SCALE.x / 2.0,
        0.0,
        20.0,
    ));
    commands
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
        .id()
}

pub fn spawn_health_bars(mut commands: Commands) {
    for handle in 0..2 {
        let container = spawn_container(
            &mut commands,
            player_spawn_transform(handle).translation,
            handle,
        );
        let background = spawn_background(&mut commands);
        let fill_container = spawn_fill_container(&mut commands);
        let fill = spawn_fill(&mut commands);

        commands.entity(fill_container).push_children(&[fill]);
        commands
            .entity(container)
            .push_children(&[fill_container, background]);
    }
}
