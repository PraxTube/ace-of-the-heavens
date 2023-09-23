use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_ggrs::*;

use crate::bullet;
use crate::input;
use crate::network::GgrsConfig;

// Movement
const MAX_SPEED: f32 = 400.0;
const MIN_SPEED: f32 = 200.0;
const DELTA_SPEED: f32 = 75.0;
const DELTA_STEERING: f32 = 3.5;
// Shooting
//const RELOAD_TIME: f32 = 0.1;
const MAX_HEALTH: f32 = 3.0;
// Misc
const PLAYER_SCALE: f32 = 1.75;
const PLAYER_RADIUS: f32 = 20.0;
const DISTANCE_FROM_SPAWN: f32 = 800.0;

#[derive(Component, Default)]
pub struct Player {
    pub handle: usize,

    pub current_speed: f32,
    health: f32,
    //shoot_timer: f32,
}

#[derive(Component, Reflect, Default)]
pub struct BulletReady(pub bool);

#[derive(Resource)]
pub struct LocalPlayerHandle(pub usize);

impl Player {
    fn new(handle: usize) -> Player {
        Player {
            handle,
            current_speed: MIN_SPEED,
            health: MAX_HEALTH,
        }
    }
}

pub fn steer_players(
    time: Res<Time>,
    inputs: Res<PlayerInputs<GgrsConfig>>,
    mut players: Query<(&mut Transform, &Player)>,
) {
    for (mut transform, player) in &mut players {
        let (input, _) = inputs[player.handle];

        let steer_direction = input::steer_direction(input);

        if steer_direction == 0.0 {
            continue;
        }

        let rotation = DELTA_STEERING * steer_direction * time.delta_seconds();
        transform.rotate_z(rotation);
    }
}

pub fn accelerate_players(
    time: Res<Time>,
    inputs: Res<PlayerInputs<GgrsConfig>>,
    mut players: Query<&mut Player>,
) {
    for mut player in &mut players {
        let (input, _) = inputs[player.handle];

        let accelerate_direction = input::accelerate_direction(input);

        if accelerate_direction == 0.0 {
            continue;
        }

        player.current_speed += DELTA_SPEED * accelerate_direction * time.delta_seconds();
        player.current_speed = player.current_speed.clamp(MIN_SPEED, MAX_SPEED);
    }
}

pub fn move_players(time: Res<Time>, mut players: Query<(&mut Transform, &Player)>) {
    for (mut transform, player) in &mut players {
        let direction = transform.local_x();
        transform.translation += direction * player.current_speed * time.delta_seconds();
    }
}

pub fn reload_bullets(
    inputs: Res<PlayerInputs<GgrsConfig>>,
    mut players: Query<(&mut BulletReady, &Player)>,
) {
    for (mut bullet_ready, player) in &mut players {
        let (input, _) = inputs[player.handle];
        if !input::fire(input) {
            bullet_ready.0 = true;
        }
    }
}

pub fn damage_players(
    mut players: Query<(&Transform, &mut Player), Without<bullet::Bullet>>,
    mut bullets: Query<(&Transform, &mut bullet::Bullet)>,
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
            if distance < PLAYER_RADIUS + bullet::BULLET_RADIUS {
                player.health -= bullet.damage;
                bullet.disabled = true;
            }
        }
    }
}

pub fn destroy_players(
    mut commands: Commands,
    players: Query<(Entity, &Player), Without<bullet::Bullet>>,
) {
    for (player_entity, player) in &players {
        if player.health <= 0.0 {
            commands.entity(player_entity).despawn_recursive();
        }
    }
}

pub fn spawn_players(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("plane1.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 1, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn((
            Player::new(0),
            BulletReady(true),
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(0),
                transform: Transform::from_scale(Vec3::splat(PLAYER_SCALE))
                    .with_translation(Vec3::new(-DISTANCE_FROM_SPAWN, 0.0, 0.0)),
                ..default()
            },
        ))
        .add_rollback();

    let texture_handle = asset_server.load("plane2.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 1, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn((
            Player::new(1),
            BulletReady(true),
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(0),
                transform: Transform::from_scale(Vec3::splat(PLAYER_SCALE))
                    .with_translation(Vec3::new(DISTANCE_FROM_SPAWN, 0.0, 0.0))
                    .with_rotation(Quat::from_rotation_z(std::f32::consts::PI)),
                ..default()
            },
        ))
        .add_rollback();
}
