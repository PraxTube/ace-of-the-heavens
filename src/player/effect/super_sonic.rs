use std::time::Duration;

use bevy::core::FrameCount;
use bevy::prelude::*;
use bevy_ggrs::AddRollbackCommandExtension;

use crate::audio::RollbackSound;
use crate::misc::utils::quat_from_vec3;
use crate::network::ggrs_config::GGRS_FPS;
use crate::player::movement::ReachedMaxSpeed;
use crate::GameAssets;

#[derive(Component)]
pub struct SuperSonicAnimationTimer {
    pub timer: Timer,
    disabled: bool,
}

impl Default for SuperSonicAnimationTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.075, TimerMode::Repeating),
            disabled: false,
        }
    }
}

pub fn spawn_super_sonic_effects(
    mut commands: Commands,
    assets: Res<GameAssets>,
    frame: Res<FrameCount>,
    mut ev_reached_max_speed: EventReader<ReachedMaxSpeed>,
) {
    for ev in &mut ev_reached_max_speed {
        let transform = Transform::from_translation(ev.position)
            .with_rotation(quat_from_vec3(ev.direction))
            .with_scale(Vec3::splat(1.5));
        let entity = commands
            .spawn((
                SuperSonicAnimationTimer::default(),
                SpriteSheetBundle {
                    transform,
                    texture_atlas: assets.super_sonic.clone(),
                    ..default()
                },
            ))
            .id();
        commands
            .spawn(RollbackSound {
                clip: assets.super_sonic_sound.clone(),
                start_frame: frame.0 as usize,
                sub_key: entity.index() as usize,
                ..default()
            })
            .add_rollback();
    }
}

pub fn animate_super_sonic_effects(
    mut query: Query<(&mut SuperSonicAnimationTimer, &mut TextureAtlasSprite)>,
) {
    for (mut timer, mut sprite) in &mut query {
        if timer.disabled {
            continue;
        }

        timer
            .timer
            .tick(Duration::from_secs_f32(1.0 / GGRS_FPS as f32));
        if timer.timer.just_finished() {
            if sprite.index == 8 {
                timer.disabled = true;
            } else {
                sprite.index += 1;
            }
        }
    }
}

pub fn despawn_super_sonic_effects(
    mut commands: Commands,
    query: Query<(Entity, &SuperSonicAnimationTimer)>,
) {
    for (entity, timer) in &query {
        if timer.disabled {
            commands.entity(entity).despawn_recursive();
        }
    }
}
