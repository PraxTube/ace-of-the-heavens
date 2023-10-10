use bevy::prelude::*;
use bevy_ggrs::prelude::*;

use super::obstacle::Obstacle;
use crate::{debug::DebugTransform, GameAssets};

const OFFSET: Vec3 = Vec3::new(0.0, 0.0, -10.0);

pub fn spawn_wall_1_1(commands: &mut Commands, spawn_position: Vec2, assets: &Res<GameAssets>) {
    let transform = Transform::from_translation(spawn_position.extend(0.0) + OFFSET);
    commands
        .spawn((
            Obstacle::new(Vec2::new(-32.0, 0.0), Vec2::new(32.0, 48.0), spawn_position),
            DebugTransform::new(&transform),
            SpriteBundle {
                texture: assets.wall_1_1.clone(),
                transform,
                ..default()
            },
        ))
        .add_rollback();
}

pub fn spawn_wall_2_2(commands: &mut Commands, spawn_position: Vec2, assets: &Res<GameAssets>) {
    let transform = Transform::from_translation(spawn_position.extend(0.0) + OFFSET);
    commands
        .spawn((
            Obstacle::new(
                Vec2::new(-64.0, -32.0),
                Vec2::new(64.0, 80.0),
                spawn_position,
            ),
            DebugTransform::new(&transform),
            SpriteBundle {
                texture: assets.wall_2_2.clone(),
                transform,
                ..default()
            },
        ))
        .add_rollback();
}

pub fn spawn_wall_1_5(commands: &mut Commands, spawn_position: Vec2, assets: &Res<GameAssets>) {
    let transform = Transform::from_translation(spawn_position.extend(0.0) + OFFSET);
    commands
        .spawn((
            Obstacle::new(
                Vec2::new(-32.0, -112.0),
                Vec2::new(32.0, 160.0),
                spawn_position,
            ),
            DebugTransform::new(&transform),
            SpriteBundle {
                texture: assets.wall_1_5.clone(),
                transform,
                ..default()
            },
        ))
        .add_rollback();
}

pub fn spawn_wall_5_1(commands: &mut Commands, spawn_position: Vec2, assets: &Res<GameAssets>) {
    let transform = Transform::from_translation(spawn_position.extend(0.0) + OFFSET);
    commands
        .spawn((
            Obstacle::new(
                Vec2::new(-160.0, 0.0),
                Vec2::new(160.0, 48.0),
                spawn_position,
            ),
            DebugTransform::new(&transform),
            SpriteBundle {
                texture: assets.wall_5_1.clone(),
                transform,
                ..default()
            },
        ))
        .add_rollback();
}

pub fn spawn_wall_1_10(commands: &mut Commands, spawn_position: Vec2, assets: &Res<GameAssets>) {
    let transform = Transform::from_translation(spawn_position.extend(0.0) + OFFSET);
    commands
        .spawn((
            Obstacle::new(
                Vec2::new(-32.0, -272.0),
                Vec2::new(32.0, 320.0),
                spawn_position,
            ),
            DebugTransform::new(&transform),
            SpriteBundle {
                texture: assets.wall_1_10.clone(),
                transform,
                ..default()
            },
        ))
        .add_rollback();
}
