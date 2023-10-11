use bevy::prelude::*;
use bevy_ggrs::prelude::*;

use super::dodge::DodgeTimer;
use super::player::Player;
use super::shooting::bullet::BulletTimer;
use super::shooting::rocket::RocketTimer;

use crate::debug::DebugTransform;
use crate::map::CollisionEntity;
use crate::GameAssets;
use crate::RollbackState;

pub const P1_TRANSFORM: Transform = Transform {
    scale: Vec3::splat(1.75),
    rotation: Quat::IDENTITY,
    translation: Vec3::new(-800.0, 0.0, 0.0),
};
pub const P2_TRANSFORM: Transform = Transform {
    scale: Vec3::splat(1.75),
    rotation: Quat::from_xyzw(0.0, 0.0, 1.0, 0.0),
    translation: Vec3::new(800.0, 0.0, 0.0),
};

pub fn player_spawn_transform(handle: usize) -> Transform {
    if handle == 0 {
        P1_TRANSFORM
    } else {
        P2_TRANSFORM
    }
}

fn spawn_player(
    commands: &mut Commands,
    texture_atlas_handle: Handle<TextureAtlas>,
    handle: usize,
) {
    let transform = player_spawn_transform(handle);
    commands
        .spawn((
            Player::new(handle),
            BulletTimer::default(),
            RocketTimer::default(),
            DodgeTimer::default(),
            CollisionEntity::default(),
            DebugTransform::new(&transform),
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(0),
                transform,
                ..default()
            },
        ))
        .add_rollback();
}

pub fn spawn_players(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    assets: Res<GameAssets>,
) {
    let texture_handles = [assets.player_1.clone(), assets.player_2.clone()];

    for (handle, texture_handle) in texture_handles.into_iter().enumerate() {
        let texture_atlas =
            TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 1, 1, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        spawn_player(&mut commands, texture_atlas_handle, handle);
    }
}

pub fn despawn_players(
    mut commands: Commands,
    mut players: Query<(Entity, &mut Player, &CollisionEntity)>,
    mut next_state: ResMut<NextState<RollbackState>>,
) {
    for (player_entity, mut player, collision_entity) in &mut players {
        if player.health <= 0 || collision_entity.disabled {
            player.health = 0;
            commands.entity(player_entity).despawn_recursive();
        }
    }

    if players.iter().count() <= 1 {
        next_state.set(RollbackState::RoundEnd);
    }
}
