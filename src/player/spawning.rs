use bevy::core::FrameCount;
use bevy::prelude::*;
use bevy_ggrs::prelude::*;
use bevy_hanabi::EffectAsset;

use super::dodge::spawn_plane_whites;
use super::dodge::DodgeTimer;
use super::effect::trail::spawn_player_trails;
use super::shooting::bullet::BulletTimer;
use super::shooting::rocket::spawn_player_wing_rockets;
use super::shooting::rocket::RocketTimer;
use super::LocalPlayerHandle;
use super::PersistentPlayerStats;
use super::Player;
use super::PlayerStats;

use crate::audio::RollbackSound;
use crate::camera::CameraShake;
use crate::debug::DebugTransform;
use crate::world::CollisionEntity;
use crate::GameAssets;
use crate::RollbackState;

pub const P1_TRANSFORM: Transform = Transform {
    scale: Vec3::ONE,
    rotation: Quat::IDENTITY,
    translation: Vec3::new(-800.0, 0.0, 0.0),
};
pub const P2_TRANSFORM: Transform = Transform {
    scale: Vec3::ONE,
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
    texture: Handle<Image>,
    handle: usize,
    stats: PlayerStats,
) -> Entity {
    let transform = player_spawn_transform(handle);
    commands
        .spawn((
            Player::new(handle, stats.clone()),
            BulletTimer::new(stats.bullet_reload_time),
            RocketTimer::new(stats.rocket_reload_time),
            DodgeTimer::new(stats.dodge_cooldown),
            CollisionEntity::default(),
            DebugTransform::new(&transform),
            SpriteBundle {
                texture,
                transform,
                ..default()
            },
        ))
        .add_rollback()
        .id()
}

pub fn spawn_players(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut effects: ResMut<Assets<EffectAsset>>,
    stats: Res<PersistentPlayerStats>,
) {
    let textures = [assets.player_1.clone(), assets.player_2.clone()];

    for (handle, texture) in textures.into_iter().enumerate() {
        let player = spawn_player(&mut commands, texture, handle, stats.stats[handle].clone());
        spawn_player_wing_rockets(&mut commands, &assets, player, handle);
        spawn_plane_whites(&mut commands, &assets, player, handle);
        spawn_player_trails(&mut commands, &mut effects, player);
    }
}

pub fn despawn_players(
    mut commands: Commands,
    players: Query<(Entity, &Player, &CollisionEntity)>,
    mut next_state: ResMut<NextState<RollbackState>>,
) {
    for (player_entity, player, collision_entity) in &players {
        if player.health == 0 || collision_entity.disabled {
            commands.entity(player_entity).despawn_recursive();
        }
    }

    if players.iter().count() <= 1 {
        next_state.set(RollbackState::RoundEnd);
    }
}

pub fn despawn_players_sound(
    mut commands: Commands,
    assets: Res<GameAssets>,
    frame: Res<FrameCount>,
    players: Query<(&Player, &CollisionEntity)>,
) {
    for (player, collision_entity) in &players {
        if player.health == 0 || collision_entity.disabled {
            commands
                .spawn(RollbackSound {
                    clip: assets.death_sound.clone(),
                    start_frame: frame.0 as usize,
                    sub_key: player.handle,
                    ..default()
                })
                .add_rollback();
        }
    }
}

pub fn despawn_players_camera_shake(
    mut camera_shake: ResMut<CameraShake>,
    players: Query<(&Player, &CollisionEntity)>,
    local_handle: Res<LocalPlayerHandle>,
) {
    for (player, collision_entity) in &players {
        if player.health == 0 || collision_entity.disabled {
            let trauma = if player.handle == local_handle.0 {
                1.0
            } else {
                0.5
            };
            camera_shake.add_trauma(trauma);
        }
    }
}
