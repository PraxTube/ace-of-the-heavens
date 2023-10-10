use bevy::prelude::*;
use rand::Rng;

use super::wall::*;
use crate::{game_logic::RNG, GameAssets};

const BORDER_MIN_X: f32 = -800.0;
const BORDER_MAX_X: f32 = 800.0;
const BORDER_MIN_Y: f32 = -448.0;
const BORDER_MAX_Y: f32 = 448.0;

pub fn spawn_background(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    assets: Res<GameAssets>,
) {
    let texture_handle = assets.background.clone();
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(1600.0, 896.0), 1, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn(SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, -1000.0)),
        ..default()
    });
}

pub fn outside_of_borders(target_position: Vec3) -> bool {
    if target_position.x < BORDER_MIN_X || target_position.x > BORDER_MAX_X {
        return true;
    } else if target_position.y < BORDER_MIN_Y || target_position.y > BORDER_MAX_Y {
        return true;
    }
    false
}

fn spawn_map_1(commands: &mut Commands, assets: Res<GameAssets>) {
    spawn_wall_1_5(commands, Vec2::new(-550.0, 200.0), &assets);
    spawn_wall_1_5(commands, Vec2::new(-550.0, -200.0), &assets);
    spawn_wall_5_1(commands, Vec2::new(-150.0, 250.0), &assets);
    spawn_wall_5_1(commands, Vec2::new(-150.0, -250.0), &assets);
    spawn_wall_2_2(commands, Vec2::new(150.0, 0.0), &assets);
    spawn_wall_1_1(commands, Vec2::new(150.0, 200.0), &assets);
}

fn spawn_map_2(commands: &mut Commands, assets: Res<GameAssets>) {
    spawn_wall_1_5(commands, Vec2::new(0.0, 0.0), &assets);
    spawn_wall_5_1(commands, Vec2::new(-450.0, -150.0), &assets);
    spawn_wall_5_1(commands, Vec2::new(450.0, 150.0), &assets);
}

fn spawn_map_3(commands: &mut Commands, assets: Res<GameAssets>) {
    spawn_wall_1_5(commands, Vec2::new(-150.0, -100.0), &assets);
    spawn_wall_1_5(commands, Vec2::new(150.0, 100.0), &assets);
}

fn spawn_map_4(commands: &mut Commands, assets: Res<GameAssets>) {
    spawn_wall_1_10(commands, Vec2::new(0.0, 0.0), &assets);
}

pub fn spawn_random_map(mut commands: Commands, assets: Res<GameAssets>, mut rng: ResMut<RNG>) {
    let index: usize = rng.0.gen_range(0..4);
    match index {
        0 => spawn_map_1(&mut commands, assets),
        1 => spawn_map_2(&mut commands, assets),
        2 => spawn_map_3(&mut commands, assets),
        3 => spawn_map_4(&mut commands, assets),
        _ => panic!("now map with this index exists, index: {}", index),
    }
}
