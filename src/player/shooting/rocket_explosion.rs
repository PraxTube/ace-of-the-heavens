use std::time::Duration;

use bevy::prelude::*;
use bevy_ggrs::*;

use crate::debug::DebugTransform;
use crate::player::player::Player;
use crate::player::player::PLAYER_RADIUS;
use crate::GameAssets;

const ROCKET_EXPLOSION_RADIUS: f32 = 100.0;
const EXPLOSTION_FRAME_LIFE: usize = 1;

#[derive(Component)]
pub struct RocketExplosion(usize, bool, usize);

#[derive(Event)]
pub struct SpawnRocketExplosion(pub Vec3, pub usize);

#[derive(Component, Reflect, Default)]
pub struct ExplosionAnimationTimer {
    pub timer: Timer,
}

impl ExplosionAnimationTimer {
    pub fn default() -> ExplosionAnimationTimer {
        ExplosionAnimationTimer {
            timer: Timer::from_seconds(0.075, TimerMode::Repeating),
        }
    }
}

pub fn spawn_rocket_explosions(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut ev_spawn_rocket_explosion: EventReader<SpawnRocketExplosion>,
) {
    let texture = assets.explosion.clone();
    let texture_atlas = TextureAtlas::from_grid(texture, Vec2::new(32.0, 32.0), 8, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    for ev in ev_spawn_rocket_explosion.iter() {
        let transform = Transform::from_translation(ev.0).with_scale(Vec3::splat(4.0));
        commands
            .spawn((
                RocketExplosion(ev.1, false, 0),
                ExplosionAnimationTimer::default(),
                DebugTransform::new(&transform),
                SpriteSheetBundle {
                    transform,
                    texture_atlas: texture_atlas_handle.clone(),
                    ..default()
                },
            ))
            .add_rollback();
    }
}

pub fn animate_rocket_explosions(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &mut ExplosionAnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (entity, mut timer, mut sprite) in &mut query {
        timer.timer.tick(Duration::from_secs_f32(1.0 / 60.0));
        if timer.timer.just_finished() {
            if sprite.index == 7 {
                commands.entity(entity).despawn_recursive();
                continue;
            }
            sprite.index += 1
        }
    }
}

pub fn check_explosion(
    mut explosions: Query<(&mut RocketExplosion, &Transform)>,
    mut players: Query<(&mut Player, &Transform)>,
) {
    for (mut rocket_explosion, rocket_transform) in &mut explosions {
        if rocket_explosion.2 > EXPLOSTION_FRAME_LIFE {
            continue;
        }
        rocket_explosion.2 += 1;

        for (mut player, player_transform) in &mut players {
            if player.handle == rocket_explosion.0 {
                continue;
            }

            let distance = Vec2::distance_squared(
                player_transform.translation.truncate(),
                rocket_transform.translation.truncate(),
            );
            if distance
                < PLAYER_RADIUS * PLAYER_RADIUS + ROCKET_EXPLOSION_RADIUS * ROCKET_EXPLOSION_RADIUS
            {
                player.health = 0;
            }
        }
    }
}
