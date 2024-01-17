pub mod gamepad;

pub use gamepad::GamepadRumble;

use bevy::{input::gamepad::*, prelude::*};
use bevy_ggrs::*;

use crate::{player::Player, GameState};

pub const INPUT_FORWARD: u8 = 1 << 0;
pub const INPUT_BACKWARD: u8 = 1 << 1;
pub const INPUT_LEFT: u8 = 1 << 2;
pub const INPUT_RIGHT: u8 = 1 << 3;
pub const INPUT_FIRE: u8 = 1 << 4;
pub const INPUT_DODGE: u8 = 1 << 5;
pub const INPUT_ROCKET: u8 = 1 << 6;
pub const INPUT_REMATCH: u8 = 1 << 7;

pub fn input(
    In(local_handle): In<ggrs::PlayerHandle>,
    keys: Res<Input<KeyCode>>,
    mouse_buttons: Res<Input<MouseButton>>,
    gamepads: Res<Gamepads>,
    button_inputs: Res<Input<GamepadButton>>,
    button_axes: Res<Axis<GamepadButton>>,
    axes: Res<Axis<GamepadAxis>>,
    players: Query<(&Transform, &Player)>,
) -> u8 {
    let mut input = 0u8;

    if keys.any_pressed([KeyCode::Up, KeyCode::W, KeyCode::K]) {
        input |= INPUT_FORWARD;
    }
    if keys.any_pressed([KeyCode::Down, KeyCode::S, KeyCode::J]) {
        input |= INPUT_BACKWARD;
    }
    if keys.any_pressed([KeyCode::Left, KeyCode::A]) {
        input |= INPUT_LEFT;
    }
    if keys.any_pressed([KeyCode::Right, KeyCode::D, KeyCode::F]) {
        input |= INPUT_RIGHT;
    }
    if keys.any_pressed([KeyCode::Space]) {
        input |= INPUT_FIRE;
    }
    if keys.any_pressed([KeyCode::ShiftLeft, KeyCode::L]) {
        input |= INPUT_DODGE;
    }
    if keys.any_pressed([KeyCode::E, KeyCode::Semicolon])
        || mouse_buttons.pressed(MouseButton::Left)
    {
        input |= INPUT_ROCKET;
    }
    if keys.pressed(KeyCode::R) {
        input |= INPUT_REMATCH;
    }

    let controller_input = gamepad::get_gamepad_input(
        &gamepads,
        &button_inputs,
        &button_axes,
        &axes,
        &players,
        local_handle,
    );
    input |= controller_input;
    input
}

pub fn steer_direction(input: u8) -> f32 {
    let mut steer_direction: f32 = 0.0;
    if input & INPUT_LEFT != 0 {
        steer_direction += 1.0;
    }
    if input & INPUT_RIGHT != 0 {
        steer_direction -= 1.0;
    }
    steer_direction
}

pub fn accelerate_direction(input: u8) -> f32 {
    let mut accelerate_direction: f32 = 0.0;
    if input & INPUT_FORWARD != 0 {
        accelerate_direction += 1.0;
    }
    if input & INPUT_BACKWARD != 0 {
        accelerate_direction -= 1.0;
    }
    accelerate_direction
}

pub fn fire(input: u8) -> bool {
    input & INPUT_FIRE != 0
}

pub fn dodge(input: u8) -> bool {
    input & INPUT_DODGE != 0
}

pub fn rocket(input: u8) -> bool {
    input & INPUT_ROCKET != 0
}

pub fn rematch(input: u8) -> bool {
    input & INPUT_REMATCH != 0
}

pub struct AceInputPlugin;

impl Plugin for AceInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (gamepad::rumble_gamepads,))
            .init_resource::<GamepadRumble>()
            .add_systems(OnExit(GameState::AssetLoading), gamepad::configure_gamepads);
    }
}
