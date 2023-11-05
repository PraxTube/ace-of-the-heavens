use bevy::{app::AppExit, input::gamepad::*, prelude::*};
use bevy_ggrs::*;

use crate::player::Player;

const INPUT_FORWARD: u8 = 1 << 0;
const INPUT_BACKWARD: u8 = 1 << 1;
const INPUT_LEFT: u8 = 1 << 2;
const INPUT_RIGHT: u8 = 1 << 3;
const INPUT_FIRE: u8 = 1 << 4;
const INPUT_DODGE: u8 = 1 << 5;
const INPUT_ROCKET: u8 = 1 << 6;
const INPUT_REMATCH: u8 = 1 << 7;

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
    if keys.any_pressed([KeyCode::Space, KeyCode::Return]) {
        input |= INPUT_FIRE;
    }
    if keys.any_pressed([KeyCode::E, KeyCode::L]) || mouse_buttons.pressed(MouseButton::Right) {
        input |= INPUT_DODGE;
    }
    if keys.any_pressed([KeyCode::Q, KeyCode::Semicolon])
        || mouse_buttons.pressed(MouseButton::Left)
    {
        input |= INPUT_ROCKET;
    }
    if keys.pressed(KeyCode::R) {
        input |= INPUT_REMATCH;
    }

    let controller_input = get_gamepad_input(
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

fn get_gamepad_input(
    gamepads: &Res<Gamepads>,
    button_inputs: &Res<Input<GamepadButton>>,
    button_axes: &Res<Axis<GamepadButton>>,
    axes: &Res<Axis<GamepadAxis>>,
    players: &Query<(&Transform, &Player)>,
    local_handle: ggrs::PlayerHandle,
) -> u8 {
    let mut input = 0u8;

    for gamepad in gamepads.iter() {
        if gamepad.id != 0 {
            continue;
        }

        if button_inputs.pressed(GamepadButton::new(gamepad, GamepadButtonType::South)) {
            input |= INPUT_FIRE;
        }
        if button_inputs.pressed(GamepadButton::new(gamepad, GamepadButtonType::West)) {
            input |= INPUT_ROCKET;
        }
        if button_inputs.pressed(GamepadButton::new(gamepad, GamepadButtonType::South))
            || button_inputs.pressed(GamepadButton::new(gamepad, GamepadButtonType::North))
        {
            input |= INPUT_REMATCH;
        }

        let l1 = button_axes
            .get(GamepadButton::new(gamepad, GamepadButtonType::LeftTrigger))
            .unwrap_or_default();
        let r1 = button_axes
            .get(GamepadButton::new(gamepad, GamepadButtonType::RightTrigger))
            .unwrap_or_default();
        if l1.abs() > 0.01 || r1.abs() > 0.01 {
            input |= INPUT_DODGE;
        }
        let left_trigger = button_axes
            .get(GamepadButton::new(gamepad, GamepadButtonType::LeftTrigger2))
            .unwrap_or_default();
        if left_trigger.abs() > 0.01 {
            input |= INPUT_BACKWARD;
        }
        let right_trigger = button_axes
            .get(GamepadButton::new(
                gamepad,
                GamepadButtonType::RightTrigger2,
            ))
            .unwrap_or_default();
        if right_trigger.abs() > 0.01 {
            input |= INPUT_FORWARD;
        }

        let left_stick_x = axes
            .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX))
            .unwrap_or_default();
        let left_stick_y = axes
            .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickY))
            .unwrap_or_default();
        let dir = Vec2::new(left_stick_x, left_stick_y).normalize_or_zero();
        if dir != Vec2::ZERO {
            for (transform, player) in players {
                if player.handle != local_handle {
                    continue;
                }

                let p_dir = transform.rotation.mul_vec3(Vec3::X).truncate();
                let angle = dir.angle_between(p_dir);
                if angle.abs() < 0.15 {
                    break;
                }

                if angle < 0.0 {
                    input |= INPUT_LEFT;
                } else {
                    input |= INPUT_RIGHT;
                }
            }
        }
        break;
    }
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

pub fn quit(
    mut exit: EventWriter<AppExit>,
    keys: Res<Input<KeyCode>>,
    gamepads: Res<Gamepads>,
    button_inputs: Res<Input<GamepadButton>>,
) {
    let mut pressed = keys.pressed(KeyCode::Q);
    for gamepad in gamepads.iter() {
        if button_inputs.pressed(GamepadButton::new(gamepad, GamepadButtonType::East)) {
            pressed = true;
        }
    }
    if pressed {
        exit.send(AppExit);
    }
}

pub fn configure_gamepads(mut settings: ResMut<GamepadSettings>) {
    // add a larger default dead-zone to all axes (ignore small inputs, round to zero)
    settings.default_axis_settings.set_deadzone_lowerbound(-0.2);
    settings.default_axis_settings.set_deadzone_upperbound(0.2);

    // for buttons (or axes treated as buttons):
    let mut button_settings = ButtonSettings::default();
    // require them to be pressed almost all the way, to count
    button_settings.set_press_threshold(0.5);

    settings.default_button_settings = button_settings;
}
