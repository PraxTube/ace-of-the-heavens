//use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::window::{PresentMode, Window};
use bevy_asset_loader::prelude::*;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_ggrs::*;
use bevy_hanabi::HanabiPlugin;
use bevy_roll_safe::prelude::*;

mod assets;
mod game_logic;
mod input;
mod map;
mod misc;
mod network;
mod player;
mod ui;

use misc::debug;
use network::GgrsConfig;
use ui::connecting_screen::ConnectingTimer;
use ui::round_start_screen::{HideScreenTimer, RoundStartTimer};

pub use assets::GameAssets;

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum GameState {
    #[default]
    AssetLoading,
    Matchmaking,
    Connecting,
    InGame,
    GameOver,
}

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default, Reflect)]
pub enum RollbackState {
    #[default]
    Setup,
    RoundStart,
    InRound,
    RoundEnd,
    GameOver,
}

fn main() {
    App::new()
        .add_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading).continue_to_state(GameState::Matchmaking),
        )
        .add_collection_to_loading_state::<_, GameAssets>(GameState::AssetLoading)
        .add_plugins(
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
        )
        .add_ggrs_plugin(
            GgrsPlugin::<GgrsConfig>::new()
                .with_input_system(input::input)
                .register_roll_state::<RollbackState>()
                .register_rollback_resource::<game_logic::RoundEndTimer>()
                .register_rollback_resource::<RoundStartTimer>()
                .register_rollback_resource::<ConnectingTimer>()
                .register_rollback_resource::<HideScreenTimer>()
                .register_rollback_component::<Transform>()
                .register_rollback_component::<debug::DebugTransform>()
                .register_rollback_component::<player::Player>()
                .register_rollback_component::<player::shooting::bullet::Bullet>()
                .register_rollback_component::<player::shooting::bullet::BulletTimer>()
                .register_rollback_component::<player::shooting::rocket::Rocket>()
                .register_rollback_component::<player::shooting::rocket::RocketTimer>()
                .register_rollback_component::<player::shooting::rocket_explosion::RocketExplosion>()
                .register_rollback_component::<player::shooting::rocket_explosion::ExplosionAnimationTimer>(),
        )
        .add_roll_state::<RollbackState>(GgrsSchedule)
        .add_plugins((
            //LogDiagnosticsPlugin::default(),
            //FrameTimeDiagnosticsPlugin::default(),
            HanabiPlugin,
            game_logic::GameLogicPlugin,
            network::AceNetworkPlugin,
            ui::AceUiPlugin,
            map::MapPlugin,
            player::PlayerPlugin,
            debug::AceDebugPlugin,
        ))
        .insert_resource(ClearColor(Color::BLACK))
        .init_resource::<RoundStartTimer>()
        .init_resource::<ConnectingTimer>()
        .init_resource::<HideScreenTimer>()
        .add_systems(Update, input::quit.run_if(in_state(GameState::Matchmaking).or_else(in_state(GameState::GameOver))))
        .run();
}
