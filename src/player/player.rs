use bevy::prelude::*;
use bevy_ggrs::*;

use crate::environment::outside_of_borders;
use crate::player::health::spawn_health_bar;
use crate::player::shooting::Bullet;
use crate::player::shooting::BulletTimer;

// Movement
pub const MAX_SPEED: f32 = 400.0 / 60.0;
pub const MIN_SPEED: f32 = 200.0 / 60.0;
pub const DELTA_SPEED: f32 = 75.0 / 60.0 / 100.0;
pub const DELTA_STEERING: f32 = 3.5 / 60.0;
// Collision
pub const PLAYER_RADIUS: f32 = 20.0;
// Health
pub const MAX_HEALTH: f32 = 20.0;
// Spawning
const PLAYER_SCALE: f32 = 1.75;
const DISTANCE_FROM_SPAWN: f32 = 800.0;

#[derive(Component, Reflect, Default)]
pub struct Player {
    pub handle: usize,

    pub current_speed: f32,
    pub health: f32,
}

impl Player {
    fn new(handle: usize) -> Player {
        Player {
            handle,
            current_speed: MIN_SPEED,
            health: MAX_HEALTH,
        }
    }
}

#[derive(Resource)]
pub struct LocalPlayerHandle(pub usize);

pub fn destroy_players(
    mut commands: Commands,
    mut players: Query<(Entity, &mut Player, &Transform), Without<Bullet>>,
    asset_server: Res<AssetServer>,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    for (player_entity, mut player, transform) in &mut players {
        if player.health <= 0.0 || outside_of_borders(transform.translation) {
            player.health = 0.0;
            commands.entity(player_entity).despawn_recursive();
        }
    }

    if players.iter().count() == 0 {
        spawn_players(commands, asset_server, texture_atlases);
    }
}

fn spawn_player(
    commands: &mut Commands,
    texture_atlas_handle: Handle<TextureAtlas>,
    handle: usize,
    spawn_position: Vec3,
    spawn_rotation: Quat,
) {
    commands
        .spawn((
            Player::new(handle),
            BulletTimer::default(),
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(0),
                transform: Transform::from_scale(Vec3::splat(PLAYER_SCALE))
                    .with_translation(spawn_position)
                    .with_rotation(spawn_rotation),
                ..default()
            },
        ))
        .add_rollback();
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

    let handle: usize = 0;
    spawn_player(
        &mut commands,
        texture_atlas_handle,
        handle,
        Vec3::new(-DISTANCE_FROM_SPAWN, 0.0, 0.0),
        Quat::from_rotation_z(0.0),
    );
    spawn_health_bar(&mut commands, handle);

    let texture_handle = asset_server.load("plane2.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 1, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let handle: usize = 1;
    spawn_player(
        &mut commands,
        texture_atlas_handle,
        handle,
        Vec3::new(DISTANCE_FROM_SPAWN, 0.0, 0.0),
        Quat::from_rotation_z(std::f32::consts::PI),
    );
    spawn_health_bar(&mut commands, handle);
}
