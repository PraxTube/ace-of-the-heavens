use std::hash::{Hash, Hasher};
use std::time::Duration;

use bevy::core::FrameCount;
use bevy::prelude::*;
use bevy_ggrs::*;
use bevy_hanabi::EffectAsset;

use crate::audio::RollbackSound;
use crate::camera::CameraShake;
use crate::debug::DebugTransform;
use crate::input;
use crate::misc::utils::quat_from_vec3;
use crate::network::ggrs_config::GGRS_FPS;
use crate::network::GgrsConfig;
use crate::world::CollisionEntity;
use crate::GameAssets;

use super::super::effect::trail::spawn_trail_effect;
use super::super::{Player, PLAYER_RADIUS};
use super::rocket_explosion::spawn_rocket_explosion;

const ROCKET_RADIUS: f32 = 1.5;
const ROCKET_MOVE_SPEED: f32 = 700.0 / 60.0;
const ROCKET_START_TIME: f32 = 0.5;
const ROCKET_PUSH_STRENGTH: f32 = 20.0;

const LEFT_WING_ROCKET_OFFSET: Vec3 = Vec3::new(8.0, 22.0, -1.0);
const RIGHT_WING_ROCKET_OFFSET: Vec3 = Vec3::new(8.0, -22.0, -1.0);

#[derive(Component)]
pub struct DummyRocket;

#[derive(Component, Reflect, Default)]
#[reflect(Hash)]
pub struct Rocket {
    left_side: bool,
    current_speed: f32,
    start_timer: Timer,
    pub handle: usize,
}

impl Rocket {
    fn new(left_side: bool, player_speed: f32, handle: usize) -> Rocket {
        Rocket {
            left_side,
            current_speed: player_speed,
            start_timer: Timer::from_seconds(ROCKET_START_TIME, TimerMode::Once),
            handle,
        }
    }
}

impl Hash for Rocket {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.current_speed.to_bits().hash(state);
        self.start_timer
            .duration()
            .as_secs_f32()
            .to_bits()
            .hash(state);
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Hash)]
pub struct RocketTimer {
    pub timer: Timer,
}

impl RocketTimer {
    pub fn new(duration: f32) -> RocketTimer {
        let mut timer = Timer::from_seconds(duration, TimerMode::Once);
        timer.tick(timer.duration());
        RocketTimer { timer }
    }
}

impl Hash for RocketTimer {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.timer.duration().as_secs_f32().to_bits().hash(state);
    }
}

fn spawn_rocket(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    frame: &Res<FrameCount>,
    effects: &mut ResMut<Assets<EffectAsset>>,
    player: &Player,
    player_transform: &Transform,
    spawn_offset: Vec3,
    left_side: bool,
) {
    let transform = Transform::from_translation(
        player_transform.translation + player_transform.rotation.mul_vec3(spawn_offset),
    )
    .with_rotation(quat_from_vec3(player_transform.local_x()));
    let texture = if player.handle == 0 {
        assets.rocket1.clone()
    } else {
        assets.rocket2.clone()
    };
    let rocket_entity = commands
        .spawn((
            Rocket::new(left_side, player.current_speed, player.handle),
            CollisionEntity::default(),
            DebugTransform::new(&transform),
            SpriteBundle {
                transform,
                texture,
                ..default()
            },
        ))
        .add_rollback()
        .id();
    commands
        .spawn(RollbackSound {
            clip: assets.rocket_spawn_sound.clone(),
            start_frame: frame.0 as usize,
            sub_key: (rocket_entity.index() + frame.0) as usize,
            volume: 0.5,
            ..default()
        })
        .add_rollback();
    let trail_effect = spawn_trail_effect(commands, effects, Vec3::ZERO);
    commands
        .entity(rocket_entity)
        .push_children(&[trail_effect]);
}

pub fn fire_rockets(
    mut commands: Commands,
    assets: Res<GameAssets>,
    frame: Res<FrameCount>,
    inputs: Res<PlayerInputs<GgrsConfig>>,
    mut players: Query<(&Transform, &Player, &mut RocketTimer)>,
    mut effects: ResMut<Assets<EffectAsset>>,
) {
    for (player_transform, player, mut rocket_timer) in &mut players {
        let (input, _) = inputs[player.handle];
        if !input::rocket(input) || !rocket_timer.timer.finished() {
            continue;
        }

        spawn_rocket(
            &mut commands,
            &assets,
            &frame,
            &mut effects,
            player,
            player_transform,
            Vec3::default(),
            true,
        );
        spawn_rocket(
            &mut commands,
            &assets,
            &frame,
            &mut effects,
            player,
            player_transform,
            Vec3::default(),
            false,
        );

        rocket_timer.timer.reset();
    }
}

pub fn move_rockets(
    mut commands: Commands,
    assets: Res<GameAssets>,
    frame: Res<FrameCount>,
    mut rockets: Query<(&mut Transform, &mut Rocket, &mut DebugTransform)>,
) {
    for (mut transform, mut rocket, mut debug_transform) in &mut rockets {
        rocket
            .start_timer
            .tick(Duration::from_secs_f32(1.0 / GGRS_FPS as f32));

        let speed = if !rocket.start_timer.finished() {
            rocket.current_speed
        } else {
            rocket.current_speed + ROCKET_MOVE_SPEED
        };
        let direction = transform.local_x();
        transform.translation += direction * speed;

        if !rocket.start_timer.finished() {
            let dir = transform.rotation.mul_vec3(Vec3::X);
            let sign = if rocket.left_side { 1.0 } else { -1.0 };
            let dir = Vec3::new(-dir.y, dir.x, 0.0) * sign;
            transform.translation += dir
                * (rocket.start_timer.duration().as_secs_f32() - rocket.start_timer.elapsed_secs())
                    .powi(2)
                * ROCKET_PUSH_STRENGTH;
        }

        if rocket.start_timer.just_finished() {
            commands
                .spawn(RollbackSound {
                    clip: assets.rocket_shot.clone(),
                    start_frame: frame.0 as usize,
                    sub_key: (if rocket.left_side { 0 } else { 1 } + frame.0) as usize,
                    volume: 0.5,
                    ..default()
                })
                .add_rollback();
        }

        debug_transform.update(&transform);
    }
}

pub fn disable_rockets(
    players: Query<(&Transform, &Player)>,
    mut rockets: Query<(&mut CollisionEntity, &Rocket, &Transform)>,
) {
    for (mut collision_entity, rocket, rocket_transform) in &mut rockets {
        for (player_transform, player) in &players {
            if player.handle == rocket.handle {
                continue;
            }
            if player.dodging {
                continue;
            }

            let distance = Vec2::distance_squared(
                player_transform.translation.truncate(),
                rocket_transform.translation.truncate(),
            );
            if distance < PLAYER_RADIUS * PLAYER_RADIUS + ROCKET_RADIUS * ROCKET_RADIUS {
                collision_entity.disabled = true;
            }
        }
    }
}

pub fn destroy_rockets(
    mut commands: Commands,
    assets: Res<GameAssets>,
    frame: Res<FrameCount>,
    mut camera_shake: ResMut<CameraShake>,
    rockets: Query<(Entity, &Rocket, &Transform, &CollisionEntity)>,
) {
    for (entity, rocket, rocket_transform, collision_entity) in &rockets {
        if !collision_entity.disabled {
            continue;
        }

        spawn_rocket_explosion(
            &mut commands,
            &assets,
            &frame,
            rocket_transform.translation,
            rocket.handle,
        );
        camera_shake.add_trauma(0.5);
        commands.entity(entity).despawn();
    }
}

fn spawn_player_wing_rocket(
    commands: &mut Commands,
    player_entity: Entity,
    texture: Handle<Image>,
    offset: Vec3,
) {
    let dummy_rocket = commands
        .spawn((
            DummyRocket,
            SpriteBundle {
                transform: Transform::from_translation(offset),
                texture,
                ..default()
            },
        ))
        .add_rollback()
        .id();
    commands
        .entity(player_entity)
        .push_children(&[dummy_rocket]);
}

pub fn spawn_player_wing_rockets(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    player: Entity,
    handle: usize,
) {
    let texture = if handle == 0 {
        assets.rocket1.clone()
    } else {
        assets.rocket2.clone()
    };

    for offset in [LEFT_WING_ROCKET_OFFSET, RIGHT_WING_ROCKET_OFFSET] {
        spawn_player_wing_rocket(commands, player, texture.clone(), offset);
    }
}

pub fn toggle_visibility_dummy_rockets(
    mut commands: Commands,
    assets: Res<GameAssets>,
    frame: Res<FrameCount>,
    mut rockets: Query<&mut Visibility, With<DummyRocket>>,
    players: Query<(&RocketTimer, &Children, &Player)>,
) {
    for (rocket_timer, children, player) in &players {
        for &child in children.iter() {
            let mut visibility = match rockets.get_mut(child) {
                Ok(r) => r,
                Err(_) => continue,
            };

            if rocket_timer.timer.just_finished() {
                commands
                    .spawn(RollbackSound {
                        clip: assets.rocket_reload.clone(),
                        start_frame: frame.0 as usize,
                        sub_key: player.handle,
                        volume: 0.35,
                        ..default()
                    })
                    .add_rollback();
            }

            if rocket_timer.timer.finished() {
                *visibility = Visibility::Inherited;
            } else {
                *visibility = Visibility::Hidden;
            }
        }
    }
}
