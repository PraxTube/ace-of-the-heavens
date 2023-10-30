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
                rocket::toggle_visibility_dummy_rockets,
                rocket::move_rockets,
                rocket_explosion::check_explosion
                    .before(rocket_explosion::animate_rocket_explosions),
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
                .distributive_run_if(
                    in_state(RollbackState::RoundStart)
                        .or_else(in_state(RollbackState::InRound))
                        .or_else(in_state(RollbackState::RoundEnd)),
                ),
        );
    }
}
