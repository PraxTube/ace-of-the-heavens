use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_asset_loader::prelude::*;
use bevy_ggrs::*;

mod bullet;
mod environment;
mod input;
mod network;
mod player;

use network::GgrsConfig;

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum GameState {
    #[default]
    AssetLoading,
    Matchmaking,
    InGame,
}

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(path = "bullet.png")]
    bullet: Handle<Image>,
}

fn main() {
    App::new()
        .add_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading).continue_to_state(GameState::Matchmaking),
        )
        .add_collection_to_loading_state::<_, ImageAssets>(GameState::AssetLoading)
        .add_plugins((DefaultPlugins.set(ImagePlugin::default_nearest()),))
        .add_ggrs_plugin(
            GgrsPlugin::<GgrsConfig>::new()
                .with_input_system(input::input)
                .register_rollback_component::<Transform>()
                .register_rollback_component::<player::BulletReady>(),
        )
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(
            OnEnter(GameState::Matchmaking),
            (
                setup,
                network::start_matchbox_socket,
                environment::spawn_background,
            ),
        )
        .add_systems(OnEnter(GameState::InGame), player::spawn_players)
        .add_systems(
            Update,
            network::wait_for_players.run_if(in_state(GameState::Matchmaking)),
        )
        .add_systems(
            GgrsSchedule,
            (
                player::accelerate_players,
                player::steer_players,
                player::move_players,
                player::reload_bullets,
                bullet::fire_bullets,
                bullet::move_bullets,
                player::kill_players,
            )
                .chain(),
        )
        .run();
}

fn setup(mut commands: Commands) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scaling_mode = ScalingMode::FixedVertical(1000.0);
    commands.spawn((camera_bundle,));
}
