use bevy::prelude::*;
use bevy_ggrs::*;

const INPUT_FORWARD: u8 = 1 << 0;
const INPUT_BACKWARD: u8 = 1 << 1;
const INPUT_LEFT: u8 = 1 << 2;
const INPUT_RIGHT: u8 = 1 << 3;
const INPUT_FIRE: u8 = 1 << 4;

pub fn input(_: In<ggrs::PlayerHandle>, keys: Res<Input<KeyCode>>) -> u8 {
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
