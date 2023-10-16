use std::hash::{Hash, Hasher};
use std::time::Duration;

use bevy::core::FrameCount;
use bevy::prelude::*;
use bevy_ggrs::*;

use crate::audio::{RollbackSound, RollbackSoundBundle};
use crate::debug::DebugTransform;
use crate::network::ggrs_config::GGRS_FPS;
use crate::player::Player;
use crate::player::PLAYER_RADIUS;
use crate::GameAssets;

const ROCKET_EXPLOSION_RADIUS: f32 = 100.0;
const EXPLOSTION_FRAME_LIFE: usize = 1;

#[derive(Component, Default, Reflect, Hash)]
pub struct RocketExplosion {
    handle: usize,
    frame: usize,
    disabled: bool,
}

impl RocketExplosion {
    fn new(handle: usize) -> RocketExplosion {
        RocketExplosion {
            handle,
            frame: 0,
            disabled: false,
        }
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Hash)]
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

impl Hash for ExplosionAnimationTimer {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.timer.duration().as_secs_f32().to_bits().hash(state);
    }
}

pub fn spawn_rocket_explosion(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    frame: &Res<FrameCount>,
    position: Vec3,
    handle: usize,
) {
    let transform = Transform::from_translation(position).with_scale(Vec3::splat(4.0));
    let explosion_entity = commands
        .spawn((
            RocketExplosion::new(handle),
            ExplosionAnimationTimer::default(),
            DebugTransform::new(&transform),
            SpriteSheetBundle {
                transform,
                texture_atlas: assets.explosion.clone(),
                ..default()
            },
        ))
        .add_rollback()
        .id();
    commands
        .spawn(RollbackSoundBundle {
            sound: RollbackSound {
                clip: assets.explosion_sound.clone(),
                start_frame: frame.0 as usize,
                sub_key: (explosion_entity.index() + frame.0) as usize,
            },
        })
        .add_rollback();
}

pub fn animate_rocket_explosions(
    mut query: Query<(
        &mut RocketExplosion,
        &mut ExplosionAnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (mut rocket_explosion, mut timer, mut sprite) in &mut query {
        if sprite.index > 7 {
            rocket_explosion.disabled = true;
            continue;
        }

        timer
            .timer
            .tick(Duration::from_secs_f32(1.0 / GGRS_FPS as f32));
        if timer.timer.just_finished() {
            if sprite.index == 7 {
                rocket_explosion.disabled = true;
            } else {
                sprite.index += 1
            }
        }
    }
}

pub fn check_explosion(
    mut explosions: Query<(&mut RocketExplosion, &Transform)>,
    mut players: Query<(&mut Player, &Transform)>,
) {
    for (mut rocket_explosion, rocket_transform) in &mut explosions {
        if rocket_explosion.frame > EXPLOSTION_FRAME_LIFE {
            continue;
        }
        rocket_explosion.frame += 1;

        for (mut player, player_transform) in &mut players {
            if player.handle == rocket_explosion.handle {
                continue;
            }
            if player.dodging {
                continue;
            }

            let distance = Vec2::distance_squared(
                player_transform.translation.truncate(),
                rocket_transform.translation.truncate(),
            );
            if distance < PLAYER_RADIUS.powi(2) + ROCKET_EXPLOSION_RADIUS.powi(2) {
                player.health = 0;
            }
        }
    }
}

pub fn despawn_rocket_explosions(mut commands: Commands, query: Query<(Entity, &RocketExplosion)>) {
    for (entity, rocket_explosion) in &query {
        if rocket_explosion.disabled {
            commands.entity(entity).despawn_recursive();
        }
    }
}
