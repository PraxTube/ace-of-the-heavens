pub mod obstacle;
pub mod wall;

pub use obstacle::CollisionEntity;

use bevy::prelude::*;
use bevy_ggrs::GgrsSchedule;
use rand::Rng;
use rand_xoshiro::rand_core::SeedableRng;

use crate::game_logic::RoundStats;
use crate::misc::GameRng;
use crate::player::InGameSet;
use crate::{game_logic::Seed, GameAssets};
use crate::{GameState, RollbackState};
use obstacle::disable_collision_entities;
use wall::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(RollbackState::RoundStart), spawn_random_map)
            .add_systems(OnExit(GameState::Connecting), spawn_background)
            .add_systems(
                GgrsSchedule,
                disable_collision_entities
                    .after(InGameSet::Spawning)
                    .before(InGameSet::Last)
                    .after(apply_state_transition::<RollbackState>)
                    .distributive_run_if(in_state(RollbackState::InRound)),
            );
    }
}

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

pub fn spawn_random_map(
    mut commands: Commands,
    assets: Res<GameAssets>,
    seed: Res<Seed>,
    round_stats: Res<RoundStats>,
) {
    let mut rng = GameRng::seed_from_u64(seed.seed + round_stats.rounds_played);
    let index: usize = rng.gen_range(0..4);
    match index {
        0 => spawn_map_1(&mut commands, assets),
        1 => spawn_map_2(&mut commands, assets),
        2 => spawn_map_3(&mut commands, assets),
        3 => spawn_map_4(&mut commands, assets),
        _ => panic!("now map with this index exists, index: {}", index),
    }
}
