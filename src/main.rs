//use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::window::{PresentMode, Window};
use bevy_asset_loader::prelude::*;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_ggrs::*;
use bevy_roll_safe::prelude::*;

mod camera;
mod debug;
mod input;
mod log;
mod map;
mod network;
mod player;
mod ui;

use network::GgrsConfig;
use ui::round_start_screen::{HideScreenTimer, RoundStartTimer};
use ui::ui::GameUiPlugin;

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

#[derive(Resource, Reflect, Deref, DerefMut)]
#[reflect(Resource)]
pub struct RoundEndTimer(Timer);

impl Default for RoundEndTimer {
    fn default() -> Self {
        RoundEndTimer(Timer::from_seconds(1.0, TimerMode::Repeating))
    }
}

#[derive(Resource, Reflect, Default, Debug)]
#[reflect(Resource)]
pub struct Score(usize, usize, Option<usize>);

#[derive(Resource, Reflect, Default, Debug)]
#[reflect(Resource)]
pub struct Rematch(bool, bool);

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(path = "plane1.png")]
    player_1: Handle<Image>,
    #[asset(path = "plane2.png")]
    player_2: Handle<Image>,
    #[asset(path = "bullet.png")]
    bullet: Handle<Image>,

    #[asset(path = "obstacle.png")]
    obstacle: Handle<Image>,
    #[asset(path = "stone-background.png")]
    background: Handle<Image>,

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
                .register_rollback_resource::<RoundEndTimer>()
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
            GameUiPlugin,
            camera::CameraPlugin,
        ))
        .insert_resource(ClearColor(Color::BLACK))
        .init_resource::<RoundEndTimer>()
        .init_resource::<RoundStartTimer>()
        .init_resource::<HideScreenTimer>()
        .init_resource::<Score>()
        .init_resource::<Rematch>()
        .add_systems(
            OnEnter(GameState::Matchmaking),
            network::start_matchbox_socket,
        )
        .add_systems(
            OnExit(GameState::Matchmaking),
            (
                map::map::spawn_background,
                map::obstacle::spawn_obstacles,
                map::obstacle::spawn_borders,
            ),
        )
        .add_systems(
            Update,
            (
                network::wait_for_players.run_if(in_state(GameState::Matchmaking)),
                network::print_events_system.run_if(in_state(GameState::InGame)),
                debug::trigger_desync.run_if(in_state(GameState::InGame)),
                input::quit.run_if(in_state(GameState::GameOver)),
            ),
        )
        .add_systems(
            OnEnter(RollbackState::RoundStart),
            (clear_world, player::player::spawn_players),
        )
        .add_systems(OnEnter(RollbackState::RoundEnd), adjust_score)
        .add_systems(
            GgrsSchedule,
            (
                player::accelerate_players,
                player::steer_players,
                player::move_players,
                player::reloading::cooldown_heat,
                player::reloading::reload_bullets,
                player::shooting::fire_bullets,
                player::shooting::move_bullets,
                player::damage_players,
                player::collision::obstacle_collision,
                player::player::destroy_players,
                player::update_health_bars,
                player::reloading::update_reload_bars,
                player::reloading::color_reload_bars,
                player::shooting::destroy_bullets,
            )
                .chain()
                .after(apply_state_transition::<RollbackState>)
                .distributive_run_if(in_state(RollbackState::InRound)),
        )
        .add_systems(
            GgrsSchedule,
            player::player::check_rematch_state
                .run_if(in_state(GameState::GameOver))
                .after(apply_state_transition::<RollbackState>),
        )
        .add_systems(
            GgrsSchedule,
            (
                round_end_timeout
                    .ambiguous_with(player::player::destroy_players)
                    .distributive_run_if(in_state(RollbackState::RoundEnd))
                    .after(apply_state_transition::<RollbackState>),
                initiate_rematch
                    .ambiguous_with(player::player::destroy_players)
                    .ambiguous_with(round_end_timeout)
                    .distributive_run_if(in_state(RollbackState::GameOver))
                    .after(apply_state_transition::<RollbackState>)
                    .after(player::player::check_rematch_state),
            ),
        )
        .run();
}

fn round_end_timeout(
    mut timer: ResMut<RoundEndTimer>,
    mut next_state: ResMut<NextState<RollbackState>>,
) {
    timer.tick(std::time::Duration::from_secs_f32(1.0 / 60.0));

    if timer.just_finished() {
        next_state.set(RollbackState::RoundStart);
    }
}

fn clear_world(
    mut commands: Commands,
    players: Query<Entity, With<player::player::Player>>,
    bullets: Query<Entity, With<player::shooting::Bullet>>,
    health_bars: Query<Entity, With<player::health::HealthBar>>,
    reload_bars: Query<Entity, With<player::reloading::ReloadBar>>,
) {
    for player in &players {
        commands.entity(player).despawn_recursive();
    }

    for bullet in &bullets {
        commands.entity(bullet).despawn_recursive();
    }

    for health_bar in &health_bars {
        commands.entity(health_bar).despawn_recursive();
    }

    for reload_bar in &reload_bars {
        commands.entity(reload_bar).despawn_recursive();
    }
}

fn adjust_score(
    players: Query<&player::player::Player>,
    mut score: ResMut<Score>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_rollback_state: ResMut<NextState<RollbackState>>,
) {
    if players.iter().count() == 0 {
        score.2 = None;
        return;
    }

    if players.single().handle == 0 {
        score.0 += 1;
        score.2 = Some(0);
    } else {
        score.1 += 1;
        score.2 = Some(1);
    }

    if score.0 == ui::ui::MAX_SCORE || score.1 == ui::ui::MAX_SCORE {
        next_game_state.set(GameState::GameOver);
        next_rollback_state.set(RollbackState::GameOver);
    }
}

fn initiate_rematch(
    mut rematch: ResMut<Rematch>,
    mut score: ResMut<Score>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_rollback_state: ResMut<NextState<RollbackState>>,
) {
    if !(rematch.0 && rematch.1) {
        return;
    }

    *rematch = Rematch::default();
    *score = Score::default();

    next_game_state.set(GameState::InGame);
    next_rollback_state.set(RollbackState::RoundStart);
}
