use std::hash::{Hash, Hasher};
use std::{f32::consts::PI, time::Duration};

use bevy::core::FrameCount;
use bevy::prelude::*;
use bevy_ggrs::{AddRollbackCommandExtension, PlayerInputs};

use crate::audio::RollbackSound;
use crate::network::ggrs_config::GGRS_FPS;
use crate::GameAssets;
use crate::{input::dodge, misc::utils::quat_from_vec3, network::GgrsConfig};

use super::Player;

const DODGE_REFRESH_TIME: f32 = 0.50;

#[derive(Component, Reflect, Default)]
#[reflect(Hash)]
pub struct DodgeTimer(Timer);

#[derive(Component)]
pub struct DodgeRefreshTimer {
    timer: Timer,
    handle: usize,
}

impl DodgeTimer {
    pub fn new(cooldown: f32) -> Self {
        let mut timer = Timer::from_seconds(cooldown, TimerMode::Once);
        timer.set_elapsed(Duration::from_secs_f32(cooldown));
        Self(timer)
    }
}

impl DodgeRefreshTimer {
    fn new(handle: usize) -> Self {
        let timer = Timer::from_seconds(DODGE_REFRESH_TIME, TimerMode::Once);
        Self { timer, handle }
    }
}

impl Hash for DodgeTimer {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.duration().as_secs_f32().to_bits().hash(state);
    }
}

pub fn start_dodging(
    mut commands: Commands,
    assets: Res<GameAssets>,
    frame: Res<FrameCount>,
    mut timers: Query<(&mut DodgeTimer, &mut Player)>,
    inputs: Res<PlayerInputs<GgrsConfig>>,
) {
    for (mut timer, mut player) in &mut timers {
        let (input, _) = inputs[player.handle];
        if !(timer.0.finished() && dodge(input)) {
            continue;
        }

        player.dodging = true;
        timer.0.reset();
        commands
            .spawn(RollbackSound {
                clip: assets.dodge_sound.clone(),
                start_frame: frame.0 as usize,
                sub_key: player.handle,
                volume: 0.35,
                ..default()
            })
            .add_rollback();
    }
}

pub fn animate_dodges(mut players: Query<(&mut Transform, &mut Player, &DodgeTimer)>) {
    for (mut transform, mut player, timer) in &mut players {
        if timer.0.elapsed_secs() > player.stats.dodge_time {
            transform.rotation = quat_from_vec3(transform.local_x());
            player.dodging = false;
            continue;
        }

        transform.rotate_local_x(2.0 * PI / player.stats.dodge_time / 60.0);
    }
}

pub fn tick_dodge_timers(
    mut commands: Commands,
    assets: Res<GameAssets>,
    frame: Res<FrameCount>,
    mut timers: Query<(&mut DodgeTimer, &Player)>,
    mut dodge_refresh_timers: Query<&mut DodgeRefreshTimer>,
) {
    for (mut timer, player) in &mut timers {
        timer.0.tick(Duration::from_secs_f32(1.0 / GGRS_FPS as f32));
        if !timer.0.just_finished() {
            continue;
        }

        for mut dodge_refresh_timer in &mut dodge_refresh_timers {
            if dodge_refresh_timer.handle != player.handle {
                continue;
            }
            if !dodge_refresh_timer.timer.finished() {
                continue;
            }
            dodge_refresh_timer.timer.reset();

            commands
                .spawn(RollbackSound {
                    clip: assets.dodge_refresh_sound.clone(),
                    start_frame: frame.0 as usize,
                    sub_key: player.handle,
                    volume: 0.35,
                    ..default()
                })
                .add_rollback();
        }
    }
}

pub fn spawn_plane_whites(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    player: Entity,
    handle: usize,
) {
    let plane_white = commands
        .spawn((
            DodgeRefreshTimer::new(handle),
            SpriteBundle {
                texture: assets.plane_white.clone(),
                sprite: Sprite {
                    color: Color::rgba(1.0, 1.0, 1.0, 0.0),
                    ..default()
                },
                ..default()
            },
        ))
        .add_rollback()
        .id();
    commands.entity(player).push_children(&[plane_white]);
}

pub fn animate_dodge_refresh(mut query: Query<(&mut DodgeRefreshTimer, &mut Sprite)>) {
    for (mut dodge_refresh_timer, mut sprite) in &mut query {
        if dodge_refresh_timer.timer.finished() {
            sprite.color.set_a(0.0);
            continue;
        }

        sprite.color.set_a(
            dodge_refresh_timer.timer.duration().as_secs_f32()
                - dodge_refresh_timer.timer.elapsed().as_secs_f32(),
        );

        dodge_refresh_timer
            .timer
            .tick(Duration::from_secs_f32(1.0 / GGRS_FPS as f32));
    }
}
