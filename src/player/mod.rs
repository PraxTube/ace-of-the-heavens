pub mod dodge;
pub mod effect;
pub mod health;
pub mod movement;
pub mod shooting;
pub mod spawning;

use std::hash::{Hash, Hasher};

use bevy::prelude::*;
use bevy_ggrs::*;

use crate::input;
use crate::network::ggrs_config::PLAYER_COUNT;
use crate::network::GgrsConfig;
use crate::world::Rematch;
use crate::RollbackState;

// Movement
pub const MIN_SPEED: f32 = 000.0 / 60.0;
pub const DELTA_SPEED: f32 = 75.0 / 60.0 / 100.0;
pub const DELTA_STEERING: f32 = 3.5 / 60.0;
// Collision
pub const PLAYER_RADIUS: f32 = 24.0;
// Color
pub const P1_COLOR: Color = Color::rgb(
    0xDF as f32 / 255.0,
    0x71 as f32 / 255.0,
    0x26 as f32 / 255.0,
);
pub const P2_COLOR: Color = Color::rgb(
    0x20 as f32 / 255.0,
    0x8E as f32 / 255.0,
    0xD9 as f32 / 255.0,
);

#[derive(Reflect, Clone)]
pub struct PlayerStats {
    pub max_speed: f32,
    pub max_health: u32,
    pub bullet_damage: u32,
    pub rocket_reload_time: f32,
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            max_speed: 400.0 / 60.0,
            max_health: 2000,
            bullet_damage: 30,
            rocket_reload_time: 2.5,
        }
    }
}

#[derive(Resource, Default)]
pub struct PersistentPlayerStats {
    pub stats: [PlayerStats; PLAYER_COUNT],
}

#[derive(Component, Reflect, Default)]
#[reflect(Hash)]
pub struct Player {
    pub handle: usize,

    pub current_speed: f32,
    pub health: u32,
    pub heat: u32,
    pub overheated: bool,
    pub dodging: bool,

    pub stats: PlayerStats,
}

impl Player {
    pub fn new(handle: usize, stats: PlayerStats) -> Player {
        Player {
            handle,
            current_speed: MIN_SPEED,
            health: stats.max_health,
            heat: 0,
            overheated: false,
            dodging: false,
            stats,
        }
    }

    pub fn speed_ratio(&self) -> f32 {
        1.0 + (self.current_speed - MIN_SPEED) / (self.stats.max_speed - MIN_SPEED)
    }
}

impl Hash for Player {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.current_speed.to_bits().hash(state);
    }
}

#[derive(Resource)]
pub struct LocalPlayerHandle(pub usize);

pub fn check_rematch_state(mut rematch: ResMut<Rematch>, inputs: Res<PlayerInputs<GgrsConfig>>) {
    if input::rematch(inputs[0].0) {
        rematch.p1 = true;
    }
    if input::rematch(inputs[1].0) {
        rematch.p2 = true;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub enum InGameSet {
    Movement,
    Dodge,
    Shooting,
    Effect,
    Health,
    Spawning,
    Last,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(RollbackState::RoundStart),
            (
                spawning::spawn_players,
                health::spawn_health_bars,
                shooting::reloading::spawn_reload_bars,
            ),
        )
        .add_event::<health::PlayerTookDamage>()
        .init_resource::<PersistentPlayerStats>()
        .add_plugins((shooting::ShootingPlugin, effect::EffectPlugin))
        .add_systems(
            GgrsSchedule,
            (
                check_rematch_state
                    .run_if(in_state(RollbackState::GameOver))
                    .after(apply_state_transition::<RollbackState>),
                shooting::rocket_explosion::animate_rocket_explosions
                    .run_if(
                        in_state(RollbackState::RoundStart)
                            .or_else(in_state(RollbackState::InRound))
                            .or_else(in_state(RollbackState::RoundEnd)),
                    )
                    .after(apply_state_transition::<RollbackState>),
            ),
        )
        .configure_sets(
            GgrsSchedule,
            (
                InGameSet::Movement,
                InGameSet::Dodge,
                InGameSet::Shooting,
                InGameSet::Health,
                InGameSet::Effect,
                InGameSet::Spawning,
                InGameSet::Last,
            )
                .chain()
                .after(apply_state_transition::<RollbackState>),
        )
        .add_systems(
            GgrsSchedule,
            (
                movement::accelerate_players,
                movement::steer_players,
                movement::move_players,
            )
                .chain()
                .in_set(InGameSet::Movement)
                .distributive_run_if(in_state(RollbackState::InRound)),
        )
        .add_systems(
            GgrsSchedule,
            (
                dodge::tick_dodge_timer,
                dodge::initiate_dodge,
                dodge::animate_dodge,
                dodge::animate_dodge_refresh,
            )
                .chain()
                .in_set(InGameSet::Dodge)
                .distributive_run_if(in_state(RollbackState::InRound)),
        )
        .add_systems(
            GgrsSchedule,
            (
                health::damage_players,
                health::move_health_bars,
                health::fill_health_bars,
            )
                .chain()
                .in_set(InGameSet::Health)
                .distributive_run_if(in_state(RollbackState::InRound)),
        )
        .add_systems(
            GgrsSchedule,
            (
                spawning::despawn_players_sound,
                spawning::despawn_players_camera_shake,
                spawning::despawn_players,
            )
                .chain()
                .in_set(InGameSet::Spawning)
                .distributive_run_if(in_state(RollbackState::InRound)),
        );
    }
}
