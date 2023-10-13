pub mod bullet;
pub mod reloading;
pub mod rocket;
pub mod rocket_explosion;

use bevy::prelude::*;
use bevy_ggrs::GgrsSchedule;

use crate::player::InGameSet;
use crate::RollbackState;

pub struct ShootingPlugin;

impl Plugin for ShootingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            GgrsSchedule,
            (
                reloading::cooldown_heat,
                reloading::reload_bullets,
                reloading::reload_rockets,
                bullet::fire_bullets,
                bullet::move_bullets,
                rocket::fire_rockets,
                rocket::move_rockets,
                rocket_explosion::check_explosion,
                reloading::move_reload_bars,
                reloading::tick_reload_bars,
                reloading::color_reload_bars,
            )
                .chain()
                .in_set(InGameSet::Shooting)
                .distributive_run_if(in_state(RollbackState::InRound)),
        )
        .add_systems(
            GgrsSchedule,
            (
                bullet::destroy_bullets,
                rocket::disable_rockets,
                rocket::destroy_rockets,
            )
                .chain()
                .in_set(InGameSet::Last)
                .distributive_run_if(in_state(RollbackState::InRound)),
        );
    }
}
