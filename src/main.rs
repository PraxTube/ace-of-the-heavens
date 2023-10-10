//use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::window::{PresentMode, Window};
use bevy_asset_loader::prelude::*;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_ggrs::*;
use bevy_hanabi::HanabiPlugin;
use bevy_roll_safe::prelude::*;

mod debug;
mod game_logic;
mod input;
mod log;
mod map;
mod network;
mod player;
mod ui;

use network::GgrsConfig;
use ui::round_start_screen::{HideScreenTimer, RoundStartTimer};

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum GameState {
    #[default]
    AssetLoading,
    Matchmaking,
    InGame,
    GameOver,
}

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default, Reflect)]
pub enum RollbackState {
    #[default]
    RoundStart,
    InRound,
    RoundEnd,
    GameOver,
}

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(path = "plane1.png")]
    player_1: Handle<Image>,
    #[asset(path = "plane2.png")]
    player_2: Handle<Image>,
    #[asset(path = "bullet.png")]
    bullet: Handle<Image>,

    #[asset(path = "map/background.png")]
    background: Handle<Image>,
    #[asset(path = "map/walls/wall-1-1.png")]
    wall_1_1: Handle<Image>,
    #[asset(path = "map/walls/wall-2-2.png")]
    wall_2_2: Handle<Image>,
    #[asset(path = "map/walls/wall-1-5.png")]
    wall_1_5: Handle<Image>,
    #[asset(path = "map/walls/wall-5-1.png")]
    wall_5_1: Handle<Image>,
    #[asset(path = "map/walls/wall-1-10.png")]
    wall_1_10: Handle<Image>,

    #[asset(path = "ui/white-pixel.png")]
    white_pixel: Handle<Image>,
    #[asset(path = "ui/score-full.png")]
    score_full: Handle<Image>,
    #[asset(path = "ui/score-empty.png")]
    score_empty: Handle<Image>,

    #[asset(path = "fonts/PressStart2P.ttf")]
    font: Handle<Font>,
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
                .register_rollback_resource::<HideScreenTimer>()
                .register_rollback_component::<Transform>()
                .register_rollback_component::<debug::DebugTransform>()
                .register_rollback_component::<player::player::Player>()
                .register_rollback_component::<player::shooting::Bullet>()
                .register_rollback_component::<player::shooting::BulletTimer>(),
        )
        .add_roll_state::<RollbackState>(GgrsSchedule)
        .add_plugins((
            //LogDiagnosticsPlugin::default(),
            //FrameTimeDiagnosticsPlugin::default(),
            ui::ui::GameUiPlugin,
            HanabiPlugin,
            player::player::PlayerPlugin,
        ))
        .insert_resource(ClearColor(Color::BLACK))
        .init_resource::<game_logic::RoundEndTimer>()
        .init_resource::<RoundStartTimer>()
        .init_resource::<HideScreenTimer>()
        .init_resource::<game_logic::Score>()
        .init_resource::<game_logic::Rematch>()
        .init_resource::<game_logic::Seeds>()
        .add_systems(
            OnEnter(GameState::Matchmaking),
            (
                game_logic::spawn_camera,
                game_logic::initiate_seed.before(network::session::start_matchbox_socket),
                network::session::start_matchbox_socket,
            ),
        )
        .add_systems(
            OnExit(GameState::Matchmaking),
            (map::map::spawn_background, debug::setup_mouse_tracking),
        )
        .add_systems(
            Update,
            (
                network::session::wait_for_players.run_if(in_state(GameState::Matchmaking)),
                network::session::wait_for_seed.run_if(in_state(GameState::InGame)),
                debug::print_events_system.run_if(in_state(GameState::InGame)),
                debug::trigger_desync.run_if(in_state(GameState::InGame)),
                debug::print_mouse_transform.run_if(in_state(GameState::InGame)),
                input::quit.run_if(in_state(GameState::Matchmaking)),
                input::quit.run_if(in_state(GameState::GameOver)),
            ),
        )
        .add_systems(
            OnEnter(RollbackState::RoundStart),
            (game_logic::clear_world, map::wall::spawn_map_1),
        )
        .add_systems(OnEnter(RollbackState::RoundEnd), game_logic::adjust_score)
        .add_systems(
            GgrsSchedule,
            (
                game_logic::round_end_timeout
                    .ambiguous_with(player::spawning::despawn_players)
                    .distributive_run_if(in_state(RollbackState::RoundEnd))
                    .after(apply_state_transition::<RollbackState>),
                game_logic::initiate_rematch
                    .ambiguous_with(player::spawning::despawn_players)
                    .ambiguous_with(game_logic::round_end_timeout)
                    .distributive_run_if(in_state(RollbackState::GameOver))
                    .after(apply_state_transition::<RollbackState>)
                    .after(player::player::check_rematch_state),
            ),
        )
        .run();
}
