use chrono::Utc;

use bevy::render::view::screenshot::ScreenshotManager;
use bevy::window::{PrimaryWindow, WindowMode};
use bevy::{prelude::*, render::camera::ScalingMode};
use noisy_bevy::simplex_noise_2d_seeded;

use crate::GameState;

const CAMERA_POSITION: Vec3 = Vec3::new(0.0, 50.0, 0.0);
const NOISE_STRENGTH: f32 = 10.0;
const TRANSLATION_SHAKE_STRENGTH: f32 = 50.0;
const ROTATION_SHAKE_STRENGTH: f32 = 2.0;

pub struct AceCameraPlugin;

impl Plugin for AceCameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            (
                toggle_full_screen,
                take_screenshot,
                camera_shake.run_if(in_state(GameState::InRollbackGame)),
            ),
        )
        .init_resource::<CameraShake>()
        .add_systems(OnEnter(GameState::MainMenu), spawn_camera);
    }
}

#[derive(Resource, Default, Reflect)]
pub struct CameraShake {
    trauma: f32,
    seed: f32,
}

impl CameraShake {
    pub fn add_trauma(&mut self, trauma: f32) {
        if self.trauma == 0.0 {
            self.seed = (Utc::now().timestamp_millis() & 0xFFFF) as f32;
        }
        self.trauma = (self.trauma + trauma.abs()).min(1.0);
    }

    fn reduce_trauma(&mut self, delta: f32) {
        self.trauma = (self.trauma - delta.abs()).max(0.0)
    }

    fn noise_value(&mut self, stack: u32) -> f32 {
        simplex_noise_2d_seeded(
            Vec2::new(self.trauma * NOISE_STRENGTH, 0.0),
            self.seed + stack as f32,
        )
    }
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::FixedVertical(1100.0);
    camera.transform.translation = CAMERA_POSITION;
    commands.spawn(camera);
}

fn camera_shake(
    mut camera: Query<&mut Transform, With<Camera2d>>,
    mut shake: ResMut<CameraShake>,
    time: Res<Time>,
) {
    let mut transform = camera.single_mut();

    let translation_offset = Vec3::new(shake.noise_value(0), shake.noise_value(1), 0.0)
        * shake.trauma.powi(2)
        * TRANSLATION_SHAKE_STRENGTH;
    let rotation_offset = Quat::from_rotation_z(
        (shake.noise_value(2) * shake.trauma.powi(2) * ROTATION_SHAKE_STRENGTH).to_radians(),
    );

    transform.translation = CAMERA_POSITION + translation_offset;
    transform.rotation = Quat::IDENTITY + rotation_offset;

    shake.reduce_trauma(time.delta_seconds());
}

fn toggle_full_screen(
    mut main_window: Query<&mut Window, With<PrimaryWindow>>,
    keys: Res<Input<KeyCode>>,
) {
    let mut window = match main_window.get_single_mut() {
        Ok(w) => w,
        Err(err) => {
            error!("there is not exactly one window, {}", err);
            return;
        }
    };

    if keys.just_pressed(KeyCode::B) {
        window.mode = if window.mode == WindowMode::Windowed {
            WindowMode::Fullscreen
        } else {
            WindowMode::Windowed
        }
    }
}

fn take_screenshot(
    keys: Res<Input<KeyCode>>,
    main_window: Query<Entity, With<PrimaryWindow>>,
    mut screenshot_manager: ResMut<ScreenshotManager>,
    mut counter: Local<u32>,
) {
    if !keys.just_pressed(KeyCode::F12) {
        return;
    }

    let path = format!("./screenshot-{}.png", *counter);
    *counter += 1;
    match screenshot_manager.save_screenshot_to_disk(main_window.single(), path) {
        Ok(()) => {}
        Err(err) => error!("failed to take screenshot, {}", err),
    }
}
