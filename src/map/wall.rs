use bevy::prelude::*;
use bevy_ggrs::prelude::*;

use super::obstacle::Obstacle;
use crate::{debug::DebugTransform, GameAssets};

const OFFSET: Vec3 = Vec3::new(0.0, 0.0, -10.0);

fn spawn_wall_1_1(commands: &mut Commands, spawn_position: Vec3, assets: &Res<GameAssets>) {
    let transform = Transform::from_translation(spawn_position + OFFSET);
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

fn spawn_wall_2_2(commands: &mut Commands, spawn_position: Vec3, assets: &Res<GameAssets>) {
    let transform = Transform::from_translation(spawn_position + OFFSET);
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

fn spawn_wall_1_5(commands: &mut Commands, spawn_position: Vec3, assets: &Res<GameAssets>) {
    let transform = Transform::from_translation(spawn_position + OFFSET);
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

fn spawn_wall_5_1(commands: &mut Commands, spawn_position: Vec3, assets: &Res<GameAssets>) {
    let transform = Transform::from_translation(spawn_position + OFFSET);
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

fn spawn_wall_1_10(commands: &mut Commands, spawn_position: Vec3, assets: &Res<GameAssets>) {
    let transform = Transform::from_translation(spawn_position + OFFSET);
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

pub fn spawn_map_1(mut commands: Commands, assets: Res<GameAssets>) {
    spawn_wall_1_5(&mut commands, Vec3::new(-550.0, 200.0, 0.0), &assets);
    spawn_wall_1_5(&mut commands, Vec3::new(-550.0, -200.0, 0.0), &assets);
    spawn_wall_5_1(&mut commands, Vec3::new(-150.0, 250.0, 0.0), &assets);
    spawn_wall_5_1(&mut commands, Vec3::new(-150.0, -250.0, 0.0), &assets);
    spawn_wall_2_2(&mut commands, Vec3::new(150.0, 0.0, 0.0), &assets);
    spawn_wall_1_1(&mut commands, Vec3::new(150.0, 200.0, 0.0), &assets);
}
