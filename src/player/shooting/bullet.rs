use std::hash::{Hash, Hasher};

use bevy::core::FrameCount;
use bevy::prelude::*;
use bevy_ggrs::*;

use crate::audio::RollbackSound;
use crate::debug::DebugTransform;
use crate::input;
use crate::misc::utils::quat_from_vec3;
use crate::network::GgrsConfig;
use crate::player::Player;
use crate::world::CollisionEntity;
use crate::GameAssets;

pub const BULLET_RADIUS: f32 = 1.0;
const BULLET_MOVE_SPEED: f32 = 350.0 / 60.0;
const DAMAGE: u32 = 75;
const BULLET_RELOAD_TIME: f32 = 0.1;
const FIRE_HEAT: u32 = 80;

const LEFT_WING_BULLET_SPAWN: Vec3 = Vec3::new(10.0, 20.0, 0.0);
const RIGHT_WING_BULLET_SPAWN: Vec3 = Vec3::new(10.0, -20.0, 0.0);

#[derive(Component, Reflect, Default)]
#[reflect(Hash)]
pub struct Bullet {
    current_speed: f32,
    pub damage: u32,
    pub handle: usize,
}

impl Bullet {
    fn new(player_speed: f32, extra_damage: u32, handle: usize) -> Bullet {
        Bullet {
            current_speed: BULLET_MOVE_SPEED + player_speed,
            damage: DAMAGE + extra_damage,
            handle,
        }
    }
}

impl Hash for Bullet {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.current_speed.to_bits().hash(state);
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Hash)]
pub struct BulletTimer {
    pub timer: Timer,
}

impl BulletTimer {
    pub fn default() -> BulletTimer {
        let mut timer = Timer::from_seconds(BULLET_RELOAD_TIME, TimerMode::Repeating);
        timer.tick(timer.duration());
        BulletTimer { timer }
    }
}

impl Hash for BulletTimer {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.timer.duration().as_secs_f32().to_bits().hash(state);
    }
}

#[derive(Event)]
pub struct BulletCollided {
    pub position: Vec3,
}

fn spawn_bullet(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    frame: &Res<FrameCount>,
    player: &Player,
    player_transform: &Transform,
    spawn_offset: Vec3,
) {
    let transform = Transform::from_translation(
        player_transform.translation + player_transform.rotation.mul_vec3(spawn_offset),
    )
    .with_rotation(quat_from_vec3(player_transform.local_x()));
    let bullet_entity = commands
        .spawn((
            Bullet::new(player.current_speed, player.speed_ratio(), player.handle),
            CollisionEntity::default(),
            DebugTransform::new(&transform),
            SpriteBundle {
                transform,
                texture: assets.bullet.clone(),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(10.0, 3.0)),
                    ..default()
                },
                ..default()
            },
        ))
        .add_rollback()
        .id();
    commands
        .spawn(RollbackSound {
            clip: assets.bullet_shot.clone(),
            start_frame: frame.0 as usize,
            sub_key: (bullet_entity.index() + frame.0) as usize,
            volume: 0.4,
        })
        .add_rollback();
}

pub fn fire_bullets(
    mut commands: Commands,
    inputs: Res<PlayerInputs<GgrsConfig>>,
    assets: Res<GameAssets>,
    frame: Res<FrameCount>,
    mut players: Query<(&Transform, &mut Player, &mut BulletTimer)>,
) {
    for (player_transform, mut player, mut bullet_timer) in &mut players {
        let (input, _) = inputs[player.handle];
        if !input::fire(input) || !bullet_timer.timer.finished() {
            continue;
        }
        if player.overheated {
            continue;
        }

        spawn_bullet(
            &mut commands,
            &assets,
            &frame,
            &player,
            player_transform,
            LEFT_WING_BULLET_SPAWN,
        );
        spawn_bullet(
            &mut commands,
            &assets,
            &frame,
            &player,
            player_transform,
            RIGHT_WING_BULLET_SPAWN,
        );

        player.heat += FIRE_HEAT;
        bullet_timer.timer.reset();
    }
}

pub fn move_bullets(mut bullets: Query<(&mut Transform, &Bullet, &mut DebugTransform)>) {
    for (mut transform, bullet, mut debug_transform) in &mut bullets {
        let direction = transform.local_x();
        transform.translation += direction * bullet.current_speed;
        debug_transform.update(&transform);
    }
}

pub fn destroy_bullets(
    mut commands: Commands,
    bullets: Query<(Entity, &Transform, &CollisionEntity), With<Bullet>>,
    mut ev_bullet_collided: EventWriter<BulletCollided>,
) {
    for (entity, transform, collision_entity) in &bullets {
        if collision_entity.disabled {
            ev_bullet_collided.send(BulletCollided {
                position: transform.translation,
            });
            commands.entity(entity).despawn_recursive();
        }
    }
}
