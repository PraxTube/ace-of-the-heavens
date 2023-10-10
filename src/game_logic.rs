use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_matchbox::prelude::PeerId;
use chrono::Utc;

use crate::map;
use crate::player;
use crate::ui;
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
    players: Query<Entity, With<player::player::Player>>,
    bullets: Query<Entity, With<player::shooting::Bullet>>,
    health_bars: Query<Entity, With<player::health::HealthBar>>,
    reload_bars: Query<Entity, With<player::reloading::ReloadBar>>,
    obstacles: Query<Entity, With<map::obstacle::Obstacle>>,
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

    for obstacle in &obstacles {
        commands.entity(obstacle).despawn_recursive();
    }
}

pub fn adjust_score(
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

pub fn initiate_rematch(
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

pub fn initiate_seed(mut seeds: ResMut<Seeds>) {
    let current_time = Utc::now().timestamp() as u32;
    seeds.0.push(Seed::new(None, current_time));
}
