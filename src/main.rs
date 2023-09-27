//use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::window::{PresentMode, Window};
use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_asset_loader::prelude::*;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_ggrs::*;

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
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: PresentMode::Fifo,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .build()
                .add_before::<bevy::asset::AssetPlugin, _>(EmbeddedAssetPlugin),
            //LogDiagnosticsPlugin::default(),
            //FrameTimeDiagnosticsPlugin::default(),
        ))
        .add_ggrs_plugin(
            GgrsPlugin::<GgrsConfig>::new()
                .with_input_system(input::input)
                .register_rollback_component::<Transform>()
                .register_rollback_component::<player::player::Player>()
                .register_rollback_component::<player::shooting::Bullet>()
                .register_rollback_component::<player::shooting::BulletTimer>(),
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
        .add_systems(OnEnter(GameState::InGame), player::player::spawn_players)
        .add_systems(
            Update,
            (
                network::wait_for_players.run_if(in_state(GameState::Matchmaking)),
                network::print_events_system.run_if(in_state(GameState::InGame)),
            ),
        )
        .add_systems(
            GgrsSchedule,
            (
                player::accelerate_players,
                player::steer_players,
                player::move_players,
                player::shooting::reload_bullets,
                player::shooting::fire_bullets,
                player::shooting::move_bullets,
                player::damage_players,
                player::player::destroy_players,
                player::update_health_bars,
                player::shooting::destroy_bullets,
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
