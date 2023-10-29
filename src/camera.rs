use rand::prelude::*;

use bevy::{prelude::*, render::camera::ScalingMode};

use crate::GameState;

const CAMERA_POSITION: Vec3 = Vec3::new(0.0, 50.0, 0.0);
const TRANSLATION_SHAKE_STRENGTH: f32 = 100.0;
const ROTATION_SHAKE_STRENGTH: f32 = 8.0;

pub struct AceCameraPlugin;

impl Plugin for AceCameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            camera_shake.run_if(in_state(GameState::InRollbackGame)),
        )
        .init_resource::<CameraShake>()
        .add_systems(OnEnter(GameState::MainMenu), spawn_camera);
    }
}

#[derive(Resource, Default, Reflect)]
pub struct CameraShake {
    trauma: f32,
}

impl CameraShake {
    pub fn add_trauma(&mut self, trauma: f32) {
        self.trauma = (self.trauma + trauma).clamp(0.0, 1.0);
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
    info!("{:?}", transform.translation);

    let mut rng = rand::thread_rng();
    let translation_offset = Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0)
        * shake.trauma.powi(2)
        * TRANSLATION_SHAKE_STRENGTH;
    let rotation_offset = Quat::from_rotation_z(
        (rng.gen_range(-1.0..1.0) * shake.trauma.powi(2) * ROTATION_SHAKE_STRENGTH).to_radians(),
    );

    transform.translation = CAMERA_POSITION + translation_offset;
    transform.rotation = Quat::IDENTITY + rotation_offset;

    shake.trauma = (shake.trauma - time.delta_seconds()).max(0.0);
}
