use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_ggrs::AddRollbackCommandExtension;

use crate::player::player::{Player, MAX_HEALTH, PLAYER_RADIUS};
use crate::player::shooting;

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

            let distance = Vec2::distance(
                player_transform.translation.xy(),
                bullet_tranform.translation.xy(),
            );
            if distance < PLAYER_RADIUS + shooting::BULLET_RADIUS {
                player.health -= bullet.damage;
                bullet.disabled = true;
            }
        }
    }
}

pub fn update_health_bars(
    mut health_bars: Query<
        (&mut Transform, &HealthBar, &Children, &mut Visibility),
        (Without<Player>, Without<HealthBarFill>),
    >,
    mut health_bar_fills: Query<
        (&mut Transform, &HealthBarFill),
        (Without<Player>, Without<HealthBar>),
    >,
    players: Query<(&Transform, &Player), Without<HealthBar>>,
) {
    for (player_transform, player) in &players {
        for (mut health_bar_transform, health_bar, children, mut health_bar_visibility) in
            &mut health_bars
        {
            if player.handle != health_bar.handle {
                continue;
            }

            health_bar_transform.translation = player_transform.translation + HEALTH_BAR_OFFSET;

            for &child in children {
                let health_bar_fill = health_bar_fills.get_mut(child);
                match health_bar_fill {
                    Ok(mut fill) => {
                        let x_fill = (player.health / MAX_HEALTH).clamp(0.0, 1.0);
                        fill.0.scale = Vec3::new(x_fill, fill.0.scale.y, fill.0.scale.z);
                        if x_fill == 0.0 {
                            *health_bar_visibility = Visibility::Hidden;
                        }
                    }
                    Err(_) => {}
                }
            }
        }
    }
}

pub fn spawn_health_bar(commands: &mut Commands, handle: usize) {
    let main = commands
        .spawn((HealthBar { handle }, SpatialBundle::default()))
        .id();
    let background = commands
        .spawn((SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.2, 0.2, 0.2),
                custom_size: Some(Vec2::new(1.0, 1.0)),
                ..default()
            },
            transform: Transform::from_scale(HEALTH_BAR_SCALE).with_translation(Vec3::new(
                HEALTH_BAR_SCALE.x / 2.0,
                0.0,
                0.0,
            )),
            ..default()
        },))
        .id();
    let outer = commands
        .spawn((HealthBarFill, SpatialBundle::default()))
        .id();
    let inner = commands
        .spawn((SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.8, 0.0, 0.0),
                custom_size: Some(Vec2::new(1.0, 1.0)),
                ..default()
            },
            transform: Transform::from_scale(HEALTH_BAR_SCALE).with_translation(Vec3::new(
                HEALTH_BAR_SCALE.x / 2.0,
                0.0,
                10.0,
            )),
            ..default()
        },))
        .add_rollback()
        .id();
    commands.entity(outer).push_children(&[inner]);
    commands.entity(main).push_children(&[outer, background]);
}
