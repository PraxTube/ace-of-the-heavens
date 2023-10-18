use bevy::core::FrameCount;
use bevy::prelude::*;
use bevy_ggrs::prelude::*;

use super::dodge::DodgeTimer;
use super::shooting::bullet::BulletTimer;
use super::shooting::rocket::RocketTimer;
use super::Player;

use crate::audio::RollbackSound;
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

fn spawn_player(commands: &mut Commands, texture: Handle<Image>, handle: usize) {
    let transform = player_spawn_transform(handle);
    commands
        .spawn((
            Player::new(handle),
            BulletTimer::default(),
            RocketTimer::default(),
            DodgeTimer::default(),
            CollisionEntity::default(),
            DebugTransform::new(&transform),
            SpriteBundle {
                texture,
                transform,
                ..default()
            },
        ))
        .add_rollback();
}

pub fn spawn_players(mut commands: Commands, assets: Res<GameAssets>) {
    let textures = [assets.player_1.clone(), assets.player_2.clone()];

    for (handle, texture) in textures.into_iter().enumerate() {
        spawn_player(&mut commands, texture, handle);
    }
}

pub fn despawn_players(
    mut commands: Commands,
    assets: Res<GameAssets>,
    frame: Res<FrameCount>,
    mut players: Query<(Entity, &mut Player, &CollisionEntity)>,
    mut next_state: ResMut<NextState<RollbackState>>,
) {
    for (player_entity, mut player, collision_entity) in &mut players {
        if player.health <= 0 || collision_entity.disabled {
            player.health = 0;
            commands.entity(player_entity).despawn_recursive();
            commands
                .spawn(RollbackSound {
                    clip: assets.death_sound.clone(),
                    start_frame: frame.0 as usize,
                    sub_key: player_entity.index() as usize,
                    ..default()
                })
                .add_rollback();
        }
    }

    if players.iter().count() <= 1 {
        next_state.set(RollbackState::RoundEnd);
    }
}
