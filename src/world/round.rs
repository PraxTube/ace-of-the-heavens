use bevy::prelude::*;
use bevy_ggrs::GgrsSchedule;

use crate::player;
use crate::RollbackState;

pub const MAX_SCORE: usize = 5;

#[derive(Resource, Reflect, Default, Debug)]
#[reflect(Resource)]
pub struct Score {
    pub p1: usize,
    pub p2: usize,
    pub last_winner: Option<usize>,
}

#[derive(Resource, Reflect, Default, Debug)]
#[reflect(Resource)]
pub struct RoundStats {
    pub rounds_played: u64,
}

#[derive(Resource, Reflect, Default, Debug)]
#[reflect(Resource)]
pub struct Rematch {
    pub p1: bool,
    pub p2: bool,
}

#[derive(Resource, Reflect, Deref, DerefMut)]
#[reflect(Resource)]
pub struct RoundEndTimer(Timer);

impl Default for RoundEndTimer {
    fn default() -> Self {
        RoundEndTimer(Timer::from_seconds(1.0, TimerMode::Repeating))
    }
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

fn reset_rematch(mut rematch: ResMut<Rematch>) {
    *rematch = Rematch::default();
}

pub struct WorldRoundPlugin;

impl Plugin for WorldRoundPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RoundEndTimer>()
            .init_resource::<Score>()
            .init_resource::<Rematch>()
            .init_resource::<RoundStats>()
            .add_systems(OnExit(RollbackState::GameOver), reset_rematch)
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
