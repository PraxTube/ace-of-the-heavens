use bevy::prelude::*;

use crate::{
    input::GamepadRumble,
    player::{shooting::rocket::Rocket, LocalPlayerHandle},
};

pub fn add_rockets_gamepad_rumble(
    mut gamepad_rumble: ResMut<GamepadRumble>,
    query: Query<&Rocket>,
    local_handle: Res<LocalPlayerHandle>,
) {
    for rocket in &query {
        if rocket.handle != local_handle.0 {
            continue;
        }

        if rocket.start_timer.percent() == 0.0 {
            gamepad_rumble.add_rumble(0.1, 0.1);
        }
    }
}
