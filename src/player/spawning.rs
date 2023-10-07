use bevy::prelude::*;
use bevy_ggrs::prelude::*;
use bevy_hanabi::prelude::*;

use super::effect::spawn_trail_effect;
use super::player::{Player, PLAYER_RADIUS};

use crate::debug::DebugTransform;
use crate::map::map::outside_of_borders;
use crate::map::obstacle::{collision, Obstacle};
use crate::GameAssets;
use crate::RollbackState;

use crate::player::health::spawn_health_bar;
use crate::player::reloading::spawn_reload_bars;
use crate::player::shooting::Bullet;
use crate::player::shooting::BulletTimer;

const PLAYER_SCALE: f32 = 1.75;
const DISTANCE_FROM_SPAWN: f32 = 800.0;

pub fn despawn_players(
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
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    assets: Res<GameAssets>,
    mut effects: ResMut<Assets<EffectAsset>>,
) {
    let texture_handle = assets.player_1.clone();
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 1, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let handle: usize = 0;
    let position = Vec3::new(-DISTANCE_FROM_SPAWN, 0.0, 0.0);
    let rotation = Quat::from_rotation_z(0.0);
    let player = spawn_player(
        &mut commands,
        texture_atlas_handle,
        handle,
        position,
        rotation,
    );
    let trail_left = spawn_trail_effect(
        &mut commands,
        Vec3::new(0.0, 15.0, -1.0),
        rotation,
        &mut effects,
    );
    let trail_right = spawn_trail_effect(
        &mut commands,
        Vec3::new(0.0, -15.0, -1.0),
        rotation,
        &mut effects,
    );
    commands
        .entity(player)
        .push_children(&[trail_left, trail_right]);
    spawn_health_bar(&mut commands, handle, position);
    spawn_reload_bars(&mut commands, handle, position);

    let texture_handle = assets.player_2.clone();
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 1, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let handle: usize = 1;
    let position = Vec3::new(DISTANCE_FROM_SPAWN, 0.0, 0.0);
    let rotation = Quat::from_rotation_z(std::f32::consts::PI);
    let player = spawn_player(
        &mut commands,
        texture_atlas_handle,
        handle,
        position,
        rotation,
    );
    let trail_left = spawn_trail_effect(
        &mut commands,
        Vec3::new(0.0, 15.0, -1.0),
        rotation,
        &mut effects,
    );
    let trail_right = spawn_trail_effect(
        &mut commands,
        Vec3::new(0.0, -15.0, -1.0),
        rotation,
        &mut effects,
    );
    commands
        .entity(player)
        .push_children(&[trail_left, trail_right]);
    spawn_health_bar(&mut commands, handle, position);
    spawn_reload_bars(&mut commands, handle, position);
}
