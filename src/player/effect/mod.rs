pub mod damage;
pub mod trail;

mod bullet;
mod super_sonic;

use bevy::prelude::*;
use bevy_ggrs::GgrsSchedule;

use crate::player::InGameSet;
use crate::{GameState, RollbackState};

pub struct EffectPlugin;

impl Plugin for EffectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnExit(GameState::Matchmaking),
            (
                damage::spawn_damage_effect_spawner,
                bullet::spawn_effect_spawner,
            ),
        )
        .add_systems(
            Update,
            (bullet::despawn_muzzle_effect, bullet::spawn_muzzle_effect)
                .chain()
                .run_if(in_state(GameState::InRollbackGame)),
        )
        .add_systems(
            Update,
            (
                super_sonic::spawn_super_sonic_effects,
                super_sonic::animate_super_sonic_effects,
                super_sonic::despawn_super_sonic_effects,
            )
                .chain()
                .run_if(in_state(GameState::InRollbackGame)),
        )
        .add_systems(
            GgrsSchedule,
            (
                damage::spawn_damage_effect,
                damage::spawn_damage_effect_sound,
                damage::add_camera_shake_damage,
                bullet::add_camera_shake_bullet_fired,
                bullet::spawn_collision_effect,
                trail::disable_trails,
                trail::toggle_plane_trail_visibilities,
                trail::despawn_trails,
            )
                .chain()
                .in_set(InGameSet::Effect)
                .distributive_run_if(not(in_state(RollbackState::Setup))),
        );
    }
}
