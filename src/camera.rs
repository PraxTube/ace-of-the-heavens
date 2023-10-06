use rand::prelude::*;
use std::time::Duration;

use bevy::{prelude::*, render::camera::ScalingMode, time::Time};

use crate::GameState;

const CAMERA_POSITION: Vec3 = Vec3::new(0.0, 50.0, 0.0);

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(OnEnter(GameState::Matchmaking), spawn_camera)
            .init_resource::<CameraShake>()
            .add_systems(Update, camera_shake.run_if(in_state(GameState::InGame)));
    }
}

#[derive(Resource)]
pub struct CameraShake {
    power: f32,
    timer: Timer,
}

impl Default for CameraShake {
    fn default() -> CameraShake {
        CameraShake {
            power: 0.0,
            timer: Timer::new(Duration::from_secs_f32(0.5), TimerMode::Once),
        }
    }
}

impl CameraShake {
    pub fn add_shake(&mut self, added_power: f32) {
        self.power += added_power;
        self.timer.reset();
    }
}

pub fn spawn_camera(mut commands: Commands) {
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
    shake.timer.tick(time.delta());
    if shake.timer.finished() {
        shake.power = 0.0;
        transform.translation = CAMERA_POSITION;
        return;
    }
    if shake.power <= 0.0 {
        return;
    }

    let mut rng = rand::thread_rng();
    let offset = Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0) * shake.power;
    transform.translation += offset;

    shake.power -= time.delta_seconds() * 10.0;
}
