use std::hash::{Hash, Hasher};

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::player::shooting::bullet::BulletTimer;
use crate::player::Player;
use crate::world::Seeds;
use crate::{GameState, RollbackState};

#[derive(Reflect, Component, Default)]
#[reflect(Hash)]
pub struct DebugVec3(Vec3);
#[derive(Reflect, Component, Default)]
#[reflect(Hash)]
pub struct DebugQuat(Quat);

/// We will store the world position of the mouse cursor here.
#[derive(Resource, Default)]
pub struct MouseWorldCoords(Vec2);

pub struct AceDebugPlugin;

impl Plugin for AceDebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                debug_state_main_menu.run_if(
                    in_state(GameState::MainMenu).and_then(not(in_state(RollbackState::Setup))),
                ),
                trigger_desync.run_if(in_state(GameState::InRollbackGame)),
                print_mouse_transform.run_if(in_state(GameState::InRollbackGame)),
            ),
        )
        .add_systems(OnExit(GameState::Matchmaking), setup_mouse_tracking);
    }
}

impl Hash for DebugVec3 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.x.to_bits().hash(state);
        self.0.y.to_bits().hash(state);
        self.0.z.to_bits().hash(state);
    }
}

impl Hash for DebugQuat {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.x.to_bits().hash(state);
        self.0.y.to_bits().hash(state);
        self.0.z.to_bits().hash(state);
        self.0.w.to_bits().hash(state);
    }
}

#[derive(Reflect, Component, Hash, Default)]
#[reflect(Hash)]
pub struct DebugTransform {
    pub translation: DebugVec3,
    pub quat: DebugQuat,
    pub scale: DebugVec3,
}

impl DebugTransform {
    pub fn update(&mut self, t: &Transform) {
        self.translation = DebugVec3(t.translation);
        self.quat = DebugQuat(t.rotation);
        self.scale = DebugVec3(t.scale);
    }

    pub fn new(t: &Transform) -> DebugTransform {
        DebugTransform {
            translation: DebugVec3(t.translation),
            quat: DebugQuat(t.rotation),
            scale: DebugVec3(t.scale),
        }
    }
}

fn debug_state_main_menu() {
    error!("the rollbackstate is not in setup. This is most likely caused by rollingback nextstate calls");
}

pub fn trigger_desync(
    keyboard_input: Res<Input<KeyCode>>,
    mut bullet_timers: Query<&mut BulletTimer, With<Player>>,
) {
    if !keyboard_input.pressed(KeyCode::ShiftLeft) {
        return;
    }

    for mut bullet_timer in &mut bullet_timers {
        bullet_timer.timer.reset();
    }
}

pub fn setup_mouse_tracking(mut commands: Commands) {
    commands.init_resource::<MouseWorldCoords>();
}

pub fn print_mouse_transform(
    mut mycoords: ResMut<MouseWorldCoords>,
    // query to get the window (so we can read the current cursor position)
    q_window: Query<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    buttons: Res<Input<MouseButton>>,
    seeds: Res<Seeds>,
) {
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }

    info!("{:?}", seeds.0);

    let (camera, camera_transform) = q_camera.single();
    let window = q_window.single();

    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        mycoords.0 = world_position;
        eprintln!(
            "Mouse World coords: X: {}, Y: {}",
            world_position.x, world_position.y
        );
    }
}
