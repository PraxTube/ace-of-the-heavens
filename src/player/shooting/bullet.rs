use std::hash::{Hash, Hasher};
use std::time::Duration;

use bevy::core::FrameCount;
use bevy::prelude::*;
use bevy_ggrs::*;

use crate::audio::RollbackSound;
use crate::debug::DebugTransform;
use crate::input;
use crate::misc::utils::quat_from_vec3;
use crate::network::ggrs_config::GGRS_FPS;
use crate::network::GgrsConfig;
use crate::player::Player;
use crate::world::CollisionEntity;
use crate::GameAssets;

use super::reloading::OVERHEAT;

pub const BULLET_RADIUS: f32 = 3.0;
const BULLET_MOVE_SPEED: f32 = 450.0 / 60.0;
const BULLET_RELOAD_TIME: f32 = 0.25;
const FIRE_HEAT: u32 = 80;

const LEFT_WING_BULLET_SPAWN: Vec3 = Vec3::new(20.0, 10.0, 0.0);
const RIGHT_WING_BULLET_SPAWN: Vec3 = Vec3::new(20.0, -10.0, 0.0);

#[derive(Component, Reflect, Default)]
#[reflect(Hash)]
pub struct Bullet {
    current_speed: f32,
    pub damage: u32,
    pub handle: usize,
}

impl Bullet {
    fn new(player_speed: f32, damage: u32, handle: usize) -> Bullet {
        Bullet {
            current_speed: BULLET_MOVE_SPEED + player_speed,
            damage,
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

#[derive(Component, Reflect)]
#[reflect(Hash)]
pub struct BulletAnimationTimer {
    pub timer: Timer,
}

impl Default for BulletAnimationTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.075, TimerMode::Repeating),
        }
    }
}

impl Hash for BulletAnimationTimer {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.timer.duration().as_secs_f32().to_bits().hash(state);
    }
}

#[derive(Event)]
pub struct BulletCollided {
    pub position: Vec3,
}

#[derive(Event)]
pub struct BulletFired {
    pub position: Vec3,
    pub direction: Vec3,
    pub handle: usize,
}

fn spawn_bullet(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    frame: &Res<FrameCount>,
    player: &Player,
    player_transform: &Transform,
    side_handle: usize,
    ev_bullet_fired: &mut EventWriter<BulletFired>,
) {
    let dir = player_transform.rotation.mul_vec3(Vec3::X);
    let (spawn_offset, direction) = if side_handle == 0 {
        (LEFT_WING_BULLET_SPAWN, Vec3::new(-dir.y, dir.x, 0.0))
    } else {
        (RIGHT_WING_BULLET_SPAWN, Vec3::new(dir.y, -dir.x, 0.0))
    };
    let transform = Transform::from_translation(
        player_transform.translation + player_transform.rotation.mul_vec3(spawn_offset),
    )
    .with_rotation(quat_from_vec3(player_transform.local_x()))
    .with_scale(Vec3::new(BULLET_RADIUS, BULLET_RADIUS, 1.0));

    ev_bullet_fired.send(BulletFired {
        position: transform.translation,
        direction,
        handle: player.handle,
    });

    let bullet_entity = commands
        .spawn((
            Bullet::new(
                player.current_speed,
                (player.stats.bullet_damage as f32 * player.speed_ratio()) as u32,
                player.handle,
            ),
            BulletAnimationTimer::default(),
            CollisionEntity::default(),
            DebugTransform::new(&transform),
            SpriteSheetBundle {
                transform,
                texture_atlas: assets.bullet.clone(),
                ..default()
            },
        ))
        .add_rollback()
        .id();
    let playback_rate = 1.0 + (player.heat as f64 / OVERHEAT as f64).powi(3) * 0.5;
    commands
        .spawn(RollbackSound {
            clip: assets.bullet_shot.clone(),
            start_frame: frame.0 as usize,
            sub_key: (bullet_entity.index() + frame.0) as usize,
            volume: 0.4,
            playback_rate,
        })
        .add_rollback();
}

pub fn fire_bullets(
    mut commands: Commands,
    inputs: Res<PlayerInputs<GgrsConfig>>,
    assets: Res<GameAssets>,
    frame: Res<FrameCount>,
    mut players: Query<(&Transform, &mut Player, &mut BulletTimer)>,
    mut ev_bullet_fired: EventWriter<BulletFired>,
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
            0,
            &mut ev_bullet_fired,
        );
        spawn_bullet(
            &mut commands,
            &assets,
            &frame,
            &player,
            player_transform,
            1,
            &mut ev_bullet_fired,
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

pub fn animate_bullets(mut query: Query<(&mut BulletAnimationTimer, &mut TextureAtlasSprite)>) {
    for (mut timer, mut sprite) in &mut query {
        timer
            .timer
            .tick(Duration::from_secs_f32(1.0 / GGRS_FPS as f32));
        if timer.timer.just_finished() {
            if sprite.index == 3 {
                sprite.index = 0;
            } else {
                sprite.index += 1;
            }
        }
    }
}
