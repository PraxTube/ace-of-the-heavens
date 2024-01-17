use bevy::prelude::*;

use crate::{
    camera::CameraShake,
    input::GamepadRumble,
    player::{shooting::bullet::BulletFired, LocalPlayerHandle},
    GameAssets,
};

#[derive(Component)]
pub struct CollisionEffectSpawner;

#[derive(Component)]
pub struct MuzzleEffect;

pub fn spawn_muzzle_effect(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut ev_bullet_fired: EventReader<BulletFired>,
) {
    let texture = assets.score_full.clone();
    for ev in ev_bullet_fired.iter() {
        commands.spawn((
            MuzzleEffect,
            SpriteBundle {
                texture: texture.clone(),
                transform: Transform::from_translation(ev.position),
                ..default()
            },
        ));
    }
}

pub fn despawn_muzzle_effect(mut commands: Commands, query: Query<Entity, With<MuzzleEffect>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn add_camera_shake_bullet_fired(
    mut camera_shake: ResMut<CameraShake>,
    local_handle: Res<LocalPlayerHandle>,
    mut ev_bullet_fired: EventReader<BulletFired>,
) {
    for ev in ev_bullet_fired.iter() {
        if ev.handle == local_handle.0 {
            camera_shake.add_trauma_with_threshold(0.125, 0.3);
        }
    }
}

pub fn add_gamepad_rumble_bullet_fired(
    mut gamepad_rumble: ResMut<GamepadRumble>,
    local_handle: Res<LocalPlayerHandle>,
    mut ev_bullet_fired: EventReader<BulletFired>,
) {
    for ev in ev_bullet_fired.iter() {
        if ev.handle == local_handle.0 {
            gamepad_rumble.add_rumble(0.1, 0.1);
        }
    }
}
