use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_ggrs::GgrsSchedule;
use bevy_matchbox::prelude::PeerId;
use chrono::Utc;
use rand_xoshiro::rand_core::SeedableRng;

use crate::map;
use crate::misc::GameRng;
use crate::network::ggrs_config::PLAYER_COUNT;
use crate::network::session::start_matchbox_socket;
use crate::player;
use crate::ui::MAX_SCORE;
use crate::{GameState, RollbackState};

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
pub struct Score(pub usize, pub usize, pub Option<usize>);

#[derive(Resource, Reflect, Default, Debug)]
#[reflect(Resource)]
pub struct Rematch(pub bool, pub bool);

#[derive(Default, Debug)]
pub struct Seed {
    pub handle: Option<PeerId>,
    pub seed: u32,
}

impl Seed {
    fn new(handle: Option<PeerId>, seed: u32) -> Seed {
        Seed { handle, seed }
    }
}

#[derive(Resource, Default, Debug)]
pub struct Seeds(pub Vec<Seed>);

#[derive(Resource, Debug)]
pub struct RNG(pub GameRng);

impl Default for RNG {
    fn default() -> RNG {
        RNG(GameRng::seed_from_u64(0))
    }
}

pub struct GameLogicPlugin;

impl Plugin for GameLogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Matchmaking),
            (spawn_camera, initiate_seed.before(start_matchbox_socket)),
        )
        .init_resource::<RoundEndTimer>()
        .init_resource::<Score>()
        .init_resource::<Rematch>()
        .init_resource::<Seeds>()
        .init_resource::<RNG>()
        .add_systems(OnExit(GameState::Connecting), setup_rng)
        .add_systems(OnEnter(GameState::InGame), reset_rematch)
        .add_systems(OnEnter(RollbackState::RoundStart), clear_world)
        .add_systems(OnEnter(RollbackState::RoundEnd), adjust_score)
        .add_systems(
            GgrsSchedule,
            (
                round_end_timeout
                    .ambiguous_with(player::spawning::despawn_players)
                    .distributive_run_if(in_state(RollbackState::RoundEnd))
                    .after(apply_state_transition::<RollbackState>),
                initiate_rematch
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

pub fn adjust_score(
    players: Query<&player::Player>,
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

    if score.0 == MAX_SCORE || score.1 == MAX_SCORE {
        next_game_state.set(GameState::GameOver);
        next_rollback_state.set(RollbackState::GameOver);
    }
}

pub fn initiate_rematch(
    rematch: Res<Rematch>,
    mut score: ResMut<Score>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_rollback_state: ResMut<NextState<RollbackState>>,
) {
    if !(rematch.0 && rematch.1) {
        return;
    }

    *score = Score::default();

    next_game_state.set(GameState::InGame);
    next_rollback_state.set(RollbackState::RoundStart);
}

pub fn reset_rematch(mut rematch: ResMut<Rematch>) {
    *rematch = Rematch::default();
}

pub fn initiate_seed(mut seeds: ResMut<Seeds>) {
    let current_time = Utc::now().timestamp() as u32;
    seeds.0.push(Seed::new(None, current_time));
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

pub fn setup_rng(mut rng: ResMut<RNG>, seeds: Res<Seeds>) {
    if seeds.0.len() != PLAYER_COUNT {
        panic!("we didn't receive the seed of our peer");
    }
    *rng = RNG(GameRng::seed_from_u64(determine_seed(&seeds) as u64));
}
