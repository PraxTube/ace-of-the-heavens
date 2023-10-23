use std::hash::{Hash, Hasher};

use bevy::core::FrameCount;
use bevy::prelude::*;
use bevy_ggrs::*;
use bevy_hanabi::EffectAsset;

use crate::audio::RollbackSound;
use crate::debug::DebugTransform;
use crate::input;
use crate::map::CollisionEntity;
use crate::misc::utils::quat_from_vec3;
use crate::network::GgrsConfig;
use crate::GameAssets;

use super::super::effect::trail::spawn_trail_effect;
use super::super::{Player, PLAYER_RADIUS};
use super::rocket_explosion::spawn_rocket_explosion;

const ROCKET_RADIUS: f32 = 1.5;
const ROCKET_MOVE_SPEED: f32 = 700.0 / 60.0;
const ROCKET_RELOAD_TIME: f32 = 3.0;

const LEFT_WING_ROCKET_OFFSET: Vec3 = Vec3::new(6.0, 20.0, -1.0);
const RIGHT_WING_ROCKET_OFFSET: Vec3 = Vec3::new(6.0, -20.0, -1.0);

#[derive(Component)]
pub struct DummyRocket;

#[derive(Component, Reflect, Default)]
#[reflect(Hash)]
pub struct Rocket {
    current_speed: f32,
    pub handle: usize,
}

impl Rocket {
    fn new(player_speed: f32, handle: usize) -> Rocket {
        Rocket {
            current_speed: ROCKET_MOVE_SPEED + player_speed,
            handle,
        }
    }
}

impl Hash for Rocket {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.current_speed.to_bits().hash(state);
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Hash)]
pub struct RocketTimer {
    pub timer: Timer,
}

impl RocketTimer {
    pub fn default() -> RocketTimer {
        let mut timer = Timer::from_seconds(ROCKET_RELOAD_TIME, TimerMode::Repeating);
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
            Rocket::new(player.current_speed, player.handle),
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
            clip: assets.rocket_shot.clone(),
            start_frame: frame.0 as usize,
            sub_key: (rocket_entity.index() + frame.0) as usize,
            volume: 0.5,
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
            &player,
            player_transform,
            Vec3::default(),
        );

        rocket_timer.timer.reset();
    }
}

pub fn move_rockets(mut rockets: Query<(&mut Transform, &Rocket, &mut DebugTransform)>) {
    for (mut transform, rocket, mut debug_transform) in &mut rockets {
        let direction = transform.local_x();
        transform.translation += direction * rocket.current_speed;
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
        .id();
    commands
        .entity(player_entity)
        .push_children(&[dummy_rocket]);
}

pub fn spawn_player_wing_rockets(
    mut commands: Commands,
    assets: Res<GameAssets>,
    players: Query<Entity, With<Player>>,
) {
    for (i, player_entity) in players.iter().enumerate() {
        let texture = if i == 0 {
            assets.rocket1.clone()
        } else {
            assets.rocket2.clone()
        };

        for offset in [LEFT_WING_ROCKET_OFFSET, RIGHT_WING_ROCKET_OFFSET] {
            spawn_player_wing_rocket(&mut commands, player_entity, texture.clone(), offset);
        }
    }
}

pub fn despawn_dummy_rockets(
    mut commands: Commands,
    mut rockets: Query<(Entity, &Parent), With<DummyRocket>>,
    parents: Query<&Transform>,
) {
    for (entity, parent) in &mut rockets {
        if parents.get(parent.get()).is_ok() {
            continue;
        }
        commands.entity(entity).despawn_recursive();
    }
}

pub fn toggle_visibility_dummy_rockets(
    mut rockets: Query<&mut Visibility, With<DummyRocket>>,
    players: Query<(&RocketTimer, &Children), With<Player>>,
) {
    for (rocket_timer, children) in &players {
        for &child in children.iter() {
            let mut visibility = match rockets.get_mut(child) {
                Ok(r) => r,
                Err(_) => continue,
            };

            if rocket_timer.timer.finished() {
                *visibility = Visibility::Inherited;
            } else {
                *visibility = Visibility::Hidden;
            }
        }
    }
}
