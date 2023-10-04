use std::hash::{Hash, Hasher};

use bevy::prelude::*;
use bevy_ggrs::*;

use crate::debug::DebugTransform;
use crate::map::map::outside_of_borders;
use crate::map::obstacle::{collision, Obstacle};
use crate::player::reloading::spawn_reload_bars;
use crate::player::shooting::Bullet;
use crate::player::shooting::BulletTimer;
use crate::RollbackState;

use super::health::spawn_health_bar;

// Movement
pub const MAX_SPEED: f32 = 400.0 / 60.0;
pub const MIN_SPEED: f32 = 200.0 / 60.0;
pub const DELTA_SPEED: f32 = 75.0 / 60.0 / 100.0;
pub const DELTA_STEERING: f32 = 3.5 / 60.0;
// Collision
pub const PLAYER_RADIUS: f32 = 24.0;
// Health
pub const MAX_HEALTH: u32 = 2000;
// Spawning
const PLAYER_SCALE: f32 = 1.75;
const DISTANCE_FROM_SPAWN: f32 = 800.0;
// Color
pub const P1_COLOR: Color = Color::rgb(
    0xDF as f32 / 255.0,
    0x71 as f32 / 255.0,
    0x26 as f32 / 255.0,
);
pub const P2_COLOR: Color = Color::rgb(
    0x20 as f32 / 255.0,
    0x8E as f32 / 255.0,
    0xD9 as f32 / 255.0,
);

#[derive(Component, Reflect, Default)]
#[reflect(Hash)]
pub struct Player {
    pub handle: usize,

    pub current_speed: f32,
    pub health: u32,
    pub heat: u32,
    pub overheated: bool,
}

impl Player {
    fn new(handle: usize) -> Player {
        Player {
            handle,
            current_speed: MIN_SPEED,
            health: MAX_HEALTH,
            heat: 0,
            overheated: false,
        }
    }

    pub fn speed_ratio(&self) -> u32 {
        ((self.current_speed - MIN_SPEED).max(0.0) / (MAX_SPEED - MIN_SPEED).max(0.0) * 100.0)
            as u32
    }
}

impl Hash for Player {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.current_speed.to_bits().hash(state);
    }
}

#[derive(Resource)]
pub struct LocalPlayerHandle(pub usize);

pub fn destroy_players(
    mut commands: Commands,
    mut players: Query<(Entity, &mut Player, &Transform), Without<Bullet>>,
    obstacles: Query<&Obstacle, (Without<Player>, Without<Bullet>)>,
    mut next_state: ResMut<NextState<RollbackState>>,
) {
    for (player_entity, mut player, transform) in &mut players {
        if player.health <= 0 || outside_of_borders(transform.translation) {
            player.health = 0;
            commands.entity(player_entity).despawn_recursive();
            continue;
        }

        for obstacle in &obstacles {
            if collision(obstacle, transform.translation, PLAYER_RADIUS) {
                player.health = 0;
                commands.entity(player_entity).despawn_recursive();
            }
        }
    }

    if players.iter().count() <= 1 {
        next_state.set(RollbackState::RoundEnd);
    }
}

fn spawn_player(
    commands: &mut Commands,
    texture_atlas_handle: Handle<TextureAtlas>,
    handle: usize,
    spawn_position: Vec3,
    spawn_rotation: Quat,
) -> Entity {
    let transform = Transform::from_scale(Vec3::splat(PLAYER_SCALE))
        .with_translation(spawn_position)
        .with_rotation(spawn_rotation);
    commands
        .spawn((
            Player::new(handle),
            BulletTimer::default(),
            DebugTransform::new(&transform),
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(0),
                transform,
                ..default()
            },
        ))
        .add_rollback()
        .id()
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
    let player = spawn_player(
        &mut commands,
        texture_atlas_handle,
        handle,
        Vec3::new(-DISTANCE_FROM_SPAWN, 0.0, 0.0),
        Quat::from_rotation_z(0.0),
    );
    let health_bar = spawn_health_bar(&mut commands);
    commands.entity(player).push_children(&[health_bar]);

    spawn_reload_bars(&mut commands, handle);

    let texture_handle = asset_server.load("plane2.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 1, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let handle: usize = 1;
    let player = spawn_player(
        &mut commands,
        texture_atlas_handle,
        handle,
        Vec3::new(DISTANCE_FROM_SPAWN, 0.0, 0.0),
        Quat::from_rotation_z(std::f32::consts::PI),
    );
    let health_bar = spawn_health_bar(&mut commands);
    commands.entity(player).push_children(&[health_bar]);

    spawn_reload_bars(&mut commands, handle);
}
