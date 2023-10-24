pub mod damage;
pub mod trail;

use bevy::prelude::*;
use bevy_ggrs::GgrsSchedule;

use crate::player::InGameSet;
use crate::{GameState, RollbackState};

pub struct EffectPlugin;

impl Plugin for EffectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnExit(GameState::Matchmaking),
            damage::spawn_damage_effect_spawner,
        )
        .add_systems(
            GgrsSchedule,
            (
                damage::spawn_damage_effect,
                trail::disable_trails,
                trail::despawn_trails,
            )
                .chain()
                .in_set(InGameSet::Effect)
                .distributive_run_if(not(in_state(RollbackState::Setup))),
        );
    }
}
