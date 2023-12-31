use std::hash::{Hash, Hasher};
use std::time::Duration;

use bevy::core::FrameCount;
use bevy::prelude::*;
use bevy_ggrs::*;

use crate::audio::RollbackSound;
use crate::debug::DebugTransform;
use crate::network::ggrs_config::GGRS_FPS;
use crate::GameAssets;

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

#[derive(Component, Reflect)]
#[reflect(Hash)]
pub struct ExplosionAnimationTimer {
    pub timer: Timer,
}

impl Default for ExplosionAnimationTimer {
    fn default() -> Self {
        Self {
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
        .spawn(RollbackSound {
            clip: assets.explosion_sound.clone(),
            start_frame: frame.0 as usize,
            sub_key: (explosion_entity.index() + frame.0) as usize,
            ..default()
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
                sprite.index += 1;
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
