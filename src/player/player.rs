use std::hash::{Hash, Hasher};

use bevy::prelude::*;
use bevy_ggrs::*;

use crate::game_logic::Rematch;
use crate::input;
use crate::network::GgrsConfig;
use crate::{GameState, RollbackState};

use crate::player as p;

// Movement
pub const MAX_SPEED: f32 = 400.0 / 60.0;
pub const MIN_SPEED: f32 = 000.0 / 60.0;
pub const DELTA_SPEED: f32 = 75.0 / 60.0 / 100.0;
pub const DELTA_STEERING: f32 = 3.5 / 60.0;
// Collision
pub const PLAYER_RADIUS: f32 = 24.0;
// Health
pub const MAX_HEALTH: u32 = 2000;
// Spawning
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

#[derive(Component, Reflect, Default)]
#[reflect(Hash)]
pub struct Player {
    pub handle: usize,

    pub current_speed: f32,
    pub health: u32,
    pub heat: u32,
    pub overheated: bool,
    pub dodging: bool,
}

impl Player {
    pub fn new(handle: usize) -> Player {
        Player {
            handle,
            current_speed: MIN_SPEED,
            health: MAX_HEALTH,
            heat: 0,
            overheated: false,
            dodging: false,
        }
    }

    pub fn speed_ratio(&self) -> u32 {
        ((self.current_speed - MIN_SPEED).max(0.0) / (MAX_SPEED - MIN_SPEED).max(0.0) * 100.0)
            as u32
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
        rematch.0 = true;
    }
    if input::rematch(inputs[1].0) {
        rematch.1 = true;
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
                p::spawning::spawn_players,
                p::health::spawn_health_bars,
                p::shooting::reloading::spawn_reload_bars,
            ),
        )
        .add_event::<p::health::PlayerTookDamage>()
        .add_systems(OnEnter(RollbackState::InRound), p::effect::activate_trails)
        .add_systems(OnExit(RollbackState::InRound), p::effect::deactivate_trails)
        .add_systems(
            OnExit(GameState::Connecting),
            (
                p::effect::spawn_damage_effect_spawner,
                p::effect::spawn_trails,
            ),
        )
        .add_systems(
            Update,
            p::effect::update_trails.run_if(in_state(GameState::InGame)),
        )
        .add_systems(
            GgrsSchedule,
            (
                check_rematch_state
                    .run_if(in_state(GameState::GameOver))
                    .after(apply_state_transition::<RollbackState>),
                p::shooting::rocket_explosion::animate_rocket_explosions
                    .run_if(in_state(GameState::InGame))
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
                p::movement::accelerate_players,
                p::movement::steer_players,
                p::movement::move_players,
            )
                .chain()
                .in_set(InGameSet::Movement)
                .distributive_run_if(in_state(RollbackState::InRound)),
        )
        .add_systems(
            GgrsSchedule,
            (
                p::dodge::tick_dodge_timer,
                p::dodge::initiate_dodge,
                p::dodge::animate_dodge,
            )
                .chain()
                .in_set(InGameSet::Dodge)
                .distributive_run_if(in_state(RollbackState::InRound)),
        )
        .add_systems(
            GgrsSchedule,
            (
                p::shooting::reloading::cooldown_heat,
                p::shooting::reloading::reload_bullets,
                p::shooting::reloading::reload_rockets,
                p::shooting::bullet::fire_bullets,
                p::shooting::bullet::move_bullets,
                p::shooting::rocket::fire_rockets,
                p::shooting::rocket::move_rockets,
                p::shooting::rocket_explosion::check_explosion,
                p::shooting::reloading::move_reload_bars,
                p::shooting::reloading::tick_reload_bars,
                p::shooting::reloading::color_reload_bars,
            )
                .chain()
                .in_set(InGameSet::Shooting)
                .distributive_run_if(in_state(RollbackState::InRound)),
        )
        .add_systems(
            GgrsSchedule,
            (p::effect::spawn_damage_effect,)
                .chain()
                .in_set(InGameSet::Effect)
                .distributive_run_if(in_state(RollbackState::InRound)),
        )
        .add_systems(
            GgrsSchedule,
            (
                p::health::damage_players,
                p::health::move_health_bars,
                p::health::fill_health_bars,
            )
                .chain()
                .in_set(InGameSet::Health)
                .distributive_run_if(in_state(RollbackState::InRound)),
        )
        .add_systems(
            GgrsSchedule,
            (p::spawning::despawn_players,)
                .chain()
                .in_set(InGameSet::Spawning)
                .distributive_run_if(in_state(RollbackState::InRound)),
        )
        .add_systems(
            GgrsSchedule,
            (
                p::shooting::bullet::destroy_bullets,
                p::shooting::rocket::disable_rockets,
                p::shooting::rocket::destroy_rockets,
            )
                .chain()
                .in_set(InGameSet::Last)
                .distributive_run_if(in_state(RollbackState::InRound)),
        );
    }
}
