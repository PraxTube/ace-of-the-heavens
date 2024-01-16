pub mod damage;

mod bullet;
mod rocket;
mod super_sonic;

use bevy::prelude::*;
use bevy_ggrs::GgrsSchedule;

use crate::player::InGameSet;
use crate::{GameState, RollbackState};

pub struct EffectPlugin;

impl Plugin for EffectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
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
                damage::add_camera_shake_damage,
                damage::add_gamepad_rumble,
                bullet::add_camera_shake_bullet_fired,
                bullet::add_gamepad_rumble_bullet_fired,
                rocket::add_rockets_gamepad_rumble,
            )
                .chain()
                .run_if(in_state(GameState::InRollbackGame)),
        )
        .add_systems(
            GgrsSchedule,
            (damage::spawn_damage_effect_sound,)
                .chain()
                .in_set(InGameSet::Effect)
                .distributive_run_if(not(in_state(RollbackState::Setup))),
        );
    }
}
