use std::time::Duration;

use bevy::{input::gamepad::*, prelude::*};
use bevy_ggrs::*;

use super::*;
use crate::player::Player;

#[derive(Resource, Default, Reflect)]
pub struct GamepadRumble {
    just_added: bool,
    intensity: f32,
    duration: f32,
}

impl GamepadRumble {
    pub fn add_rumble(&mut self, intensity: f32, duration: f32) {
        self.just_added = true;
        self.intensity = intensity.clamp(0.0, 1.0);
        self.duration = duration;
    }
}

pub fn get_gamepad_input(
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

pub fn rumble_gamepads(
    gamepads: Res<Gamepads>,
    mut rumble_requests: EventWriter<GamepadRumbleRequest>,
    mut gamepad_rumble: ResMut<GamepadRumble>,
) {
    if !gamepad_rumble.just_added {
        return;
    }
    gamepad_rumble.just_added = false;

    for gamepad in gamepads.iter() {
        rumble_requests.send(GamepadRumbleRequest::Add {
            gamepad,
            intensity: GamepadRumbleIntensity {
                // intensity low-frequency motor, usually on the left-hand side
                strong_motor: gamepad_rumble.intensity,
                // intensity of high-frequency motor, usually on the right-hand side
                weak_motor: gamepad_rumble.intensity,
            },
            duration: Duration::from_secs_f32(gamepad_rumble.duration),
        });

        if gamepad_rumble.duration == 0.0 || gamepad_rumble.intensity == 0.0 {
            rumble_requests.send(GamepadRumbleRequest::Stop { gamepad });
        }
    }
}
