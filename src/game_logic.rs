use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_ggrs::{GgrsSchedule, Session};
use bevy_matchbox::prelude::PeerId;
use chrono::Utc;

use crate::audio::BgmStage;
use crate::audio::PlaybackStates;
use crate::map;
use crate::network::ggrs_config::PLAYER_COUNT;
use crate::network::session::{start_matchbox_socket, Ready};
use crate::network::socket::AceSocket;
use crate::network::GgrsConfig;
use crate::player::{self, LocalPlayerHandle};
use crate::ui::round_start_screen::{HideScreenTimer, RoundStartTimer};
use crate::ui::MAX_SCORE;
use crate::{GameState, RollbackState};

#[derive(Resource, Reflect, Deref, DerefMut, Clone)]
#[reflect(Resource)]
pub struct RoundEndTimer(Timer);

impl Default for RoundEndTimer {
    fn default() -> Self {
        RoundEndTimer(Timer::from_seconds(1.0, TimerMode::Repeating))
    }
}

#[derive(Resource, Reflect, Default, Debug, Clone)]
#[reflect(Resource)]
pub struct Score {
    pub p1: usize,
    pub p2: usize,
    pub last_winner: Option<usize>,
}

#[derive(Resource, Reflect, Default, Debug)]
#[reflect(Resource)]
pub struct Rematch {
    pub p1: bool,
    pub p2: bool,
}

#[derive(Resource, Reflect, Default, Debug)]
#[reflect(Resource)]
pub struct RoundStats {
    pub rounds_played: u64,
}

#[derive(Default, Debug)]
pub struct SeedHandle {
    pub handle: Option<PeerId>,
    pub seed: u32,
}

impl SeedHandle {
    fn new(handle: Option<PeerId>, seed: u32) -> SeedHandle {
        SeedHandle { handle, seed }
    }
}

#[derive(Resource, Default, Debug)]
pub struct Seeds(pub Vec<SeedHandle>);

#[derive(Resource, Default, Debug)]
pub struct Seed {
    pub seed: u64,
}

pub struct GameLogicPlugin;

impl Plugin for GameLogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::MainMenu),
            (
                purge_entities,
                reset_resources,
                purge_network_resources,
                spawn_camera,
            )
                .chain(),
        )
        .add_systems(
            OnEnter(GameState::Matchmaking),
            initiate_seed.before(start_matchbox_socket),
        )
        .init_resource::<RoundEndTimer>()
        .init_resource::<Score>()
        .init_resource::<Rematch>()
        .init_resource::<RoundStats>()
        .init_resource::<Seeds>()
        .init_resource::<Seed>()
        .add_systems(OnExit(GameState::Matchmaking), setup_seed)
        .add_systems(OnExit(RollbackState::GameOver), reset_rematch)
        .add_systems(OnEnter(RollbackState::RoundStart), clear_world)
        .add_systems(OnEnter(RollbackState::RoundEnd), adjust_score)
        .add_systems(
            GgrsSchedule,
            (
                round_end_timeout
                    .ambiguous_with(player::spawning::despawn_players)
                    .distributive_run_if(in_state(RollbackState::RoundEnd))
                    .after(apply_state_transition::<RollbackState>),
                check_rematch
                    .ambiguous_with(player::spawning::despawn_players)
                    .ambiguous_with(round_end_timeout)
                    .distributive_run_if(in_state(RollbackState::GameOver))
                    .after(apply_state_transition::<RollbackState>)
                    .after(player::check_rematch_state),
            ),
        );
    }
}

pub fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::FixedVertical(1100.0);
    camera.transform.translation = Vec3::new(0.0, 50.0, 0.0);
    commands.spawn(camera);
}

pub fn round_end_timeout(
    mut timer: ResMut<RoundEndTimer>,
    mut next_state: ResMut<NextState<RollbackState>>,
) {
    timer.tick(std::time::Duration::from_secs_f32(1.0 / 60.0));

    if timer.just_finished() {
        next_state.set(RollbackState::RoundStart);
    }
}

pub fn clear_world(
    mut commands: Commands,
    players: Query<Entity, With<player::Player>>,
    bullets: Query<Entity, With<player::shooting::bullet::Bullet>>,
    rockets: Query<Entity, With<player::shooting::rocket::Rocket>>,
    rocket_explosions: Query<Entity, With<player::shooting::rocket_explosion::RocketExplosion>>,
    health_bars: Query<Entity, With<player::health::HealthBar>>,
    reload_bars: Query<Entity, With<player::shooting::reloading::ReloadBar>>,
    obstacles: Query<Entity, With<map::obstacle::Obstacle>>,
) {
    for player in &players {
        commands.entity(player).despawn_recursive();
    }

    for bullet in &bullets {
        commands.entity(bullet).despawn_recursive();
    }

    for rocket in &rockets {
        commands.entity(rocket).despawn_recursive();
    }

    for rocket_explosion in &rocket_explosions {
        commands.entity(rocket_explosion).despawn_recursive();
    }

    for health_bar in &health_bars {
        commands.entity(health_bar).despawn_recursive();
    }

    for reload_bar in &reload_bars {
        commands.entity(reload_bar).despawn_recursive();
    }

    for obstacle in &obstacles {
        commands.entity(obstacle).despawn_recursive();
    }
}

pub fn purge_entities(
    mut commands: Commands,
    entities: Query<Entity, (Without<Window>, Without<BgmStage>)>,
) {
    warn!("initiate the purge");

    for entity in &entities {
        // We use despawn instead of despawn_recursive because that would
        // result in the children being despawned but still in the query
        commands.entity(entity).despawn();
    }
}

pub fn reset_resources(
    mut round_stats: ResMut<RoundStats>,
    mut seeds: ResMut<Seeds>,
    mut score: ResMut<Score>,
    mut round_end_timer: ResMut<RoundEndTimer>,
    mut round_start_timer: ResMut<RoundStartTimer>,
    mut hide_screen_timer: ResMut<HideScreenTimer>,
    mut playback_states: ResMut<PlaybackStates>,
    mut ready: ResMut<Ready>,
) {
    *round_stats = RoundStats::default();
    *seeds = Seeds::default();
    *score = Score::default();
    *round_end_timer = RoundEndTimer::default();
    *round_start_timer = RoundStartTimer::default();
    *hide_screen_timer = HideScreenTimer::default();
    *playback_states = PlaybackStates::default();
    *ready = Ready::default();
}

pub fn purge_network_resources(world: &mut World) {
    if world.contains_resource::<AceSocket>() {
        world.remove_resource::<AceSocket>();
    }
    if world.contains_resource::<Session<GgrsConfig>>() {
        world.remove_resource::<Session<GgrsConfig>>();
    }
    if world.contains_resource::<LocalPlayerHandle>() {
        world.remove_resource::<LocalPlayerHandle>();
    }
}

pub fn adjust_score(
    players: Query<&player::Player>,
    mut score: ResMut<Score>,
    mut round_stats: ResMut<RoundStats>,
    mut next_rollback_state: ResMut<NextState<RollbackState>>,
) {
    round_stats.rounds_played += 1;
    if players.iter().count() == 0 {
        score.last_winner = None;
        return;
    }

    if players.single().handle == 0 {
        score.p1 += 1;
        score.last_winner = Some(0);
    } else {
        score.p2 += 1;
        score.last_winner = Some(1);
    }

    if score.p1 == MAX_SCORE || score.p2 == MAX_SCORE {
        next_rollback_state.set(RollbackState::GameOver);
    }
}

pub fn check_rematch(
    rematch: Res<Rematch>,
    mut score: ResMut<Score>,
    mut next_rollback_state: ResMut<NextState<RollbackState>>,
) {
    if !(rematch.p1 && rematch.p2) {
        return;
    }

    *score = Score::default();

    next_rollback_state.set(RollbackState::RoundStart);
}

pub fn reset_rematch(mut rematch: ResMut<Rematch>) {
    *rematch = Rematch::default();
}

pub fn initiate_seed(mut seeds: ResMut<Seeds>) {
    let current_time = Utc::now().timestamp() as u32;
    seeds.0.push(SeedHandle::new(None, current_time));
}

pub fn determine_seed(seeds: &Res<Seeds>) -> u32 {
    let mut smallest_seed = seeds.0[0].seed;
    for seed in &seeds.0 {
        if seed.seed < smallest_seed {
            smallest_seed = seed.seed;
        }
    }
    smallest_seed
}

pub fn setup_seed(mut seed: ResMut<Seed>, seeds: Res<Seeds>) {
    if seeds.0.len() != PLAYER_COUNT {
        panic!(
            "we didn't receive the correct amount of seeds from our peer\nReceived {} seeds",
            seeds.0.len()
        );
    }
    *seed = Seed {
        seed: determine_seed(&seeds) as u64,
    };
}
