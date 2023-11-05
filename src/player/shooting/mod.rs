pub mod bullet;
pub mod reloading;
pub mod rocket;
pub mod rocket_explosion;

mod bullet_casing;

use bevy::prelude::*;
use bevy_ggrs::GgrsSchedule;

use crate::player::InGameSet;
use crate::{GameState, RollbackState};

pub struct ShootingPlugin;

impl Plugin for ShootingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                bullet_casing::spawn_bullet_casings,
                bullet_casing::animate_bullet_casings,
                bullet_casing::despawn_bullet_casing_component,
            )
                .run_if(in_state(GameState::InRollbackGame)),
        )
        .add_systems(
            GgrsSchedule,
            (
                bullet::animate_bullets,
                rocket_explosion::animate_rocket_explosions,
            )
                .chain()
                .run_if(not(in_state(RollbackState::Setup)))
                .after(apply_state_transition::<RollbackState>),
        )
        .add_systems(
            GgrsSchedule,
            (
                reloading::cooldown_heat,
                reloading::reload_bullets,
                reloading::reload_rockets,
                bullet::fire_bullets,
                bullet::move_bullets,
                rocket::fire_rockets,
                rocket::toggle_visibility_dummy_rockets,
                rocket::update_rocket_targets,
                rocket::move_rockets,
                reloading::move_reload_bars,
                reloading::tick_reload_bars,
                reloading::color_reload_bars,
            )
                .chain()
                .in_set(InGameSet::Shooting)
                .distributive_run_if(in_state(RollbackState::InRound)),
        )
        .add_event::<bullet::BulletCollided>()
        .add_event::<bullet::BulletFired>()
        .add_systems(
            GgrsSchedule,
            (
                bullet::destroy_bullets,
                rocket::disable_rockets,
                rocket::destroy_rockets,
                rocket_explosion::despawn_rocket_explosions
                    .after(rocket_explosion::animate_rocket_explosions),
            )
                .chain()
                .in_set(InGameSet::Last)
                .distributive_run_if(not(in_state(RollbackState::Setup))),
        );
    }
}
