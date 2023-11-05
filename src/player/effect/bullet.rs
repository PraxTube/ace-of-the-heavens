use bevy::prelude::*;
use bevy_hanabi::prelude::*;

use crate::{
    camera::CameraShake,
    input::GamepadRumble,
    player::{
        shooting::bullet::{BulletCollided, BulletFired},
        LocalPlayerHandle,
    },
    GameAssets,
};

#[derive(Component)]
pub struct CollisionEffectSpawner;

#[derive(Component)]
pub struct MuzzleEffect;

pub fn spawn_effect_spawner(mut commands: Commands, mut effects: ResMut<Assets<EffectAsset>>) {
    let mut scale_gradient = Gradient::new();
    scale_gradient.add_key(0.0, Vec2::ONE * 5.0);
    scale_gradient.add_key(0.5, Vec2::ONE * 3.5);
    scale_gradient.add_key(1.0, Vec2::new(0.0, 0.0));

    let spawner = Spawner::once(10.0.into(), false);

    let writer = ExprWriter::new();

    // Give a bit of variation by randomizing the age per particle. This will
    // control the starting color and starting size of particles.
    let age = writer.lit(0.).uniform(writer.lit(0.2)).expr();
    let init_age = SetAttributeModifier::new(Attribute::AGE, age);

    // Give a bit of variation by randomizing the lifetime per particle
    let lifetime = writer.lit(0.2).uniform(writer.lit(0.4)).expr();
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    // Add constant downward acceleration to simulate gravity
    let accel = writer.lit(Vec3::Y * -8.).expr();
    let update_accel = AccelModifier::new(accel);

    // Add drag to make particles slow down a bit after the initial explosion
    let drag = writer.lit(0.8).expr();
    let update_drag = LinearDragModifier::new(drag);

    let init_pos = SetPositionSphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        radius: writer.lit(0.5).expr(),
        dimension: ShapeDimension::Volume,
    };

    // Give a bit of variation by randomizing the initial speed
    let init_vel = SetVelocitySphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        speed: (writer.rand(ScalarType::Float) * writer.lit(50.) + writer.lit(80.)).expr(),
    };

    let effect = effects.add(
        EffectAsset::new(1200, spawner, writer.finish())
            .init(init_pos)
            .init(init_vel)
            .init(init_age)
            .init(init_lifetime)
            .update(update_drag)
            .update(update_accel)
            .render(SizeOverLifetimeModifier {
                gradient: scale_gradient,
                screen_space_size: false,
            }),
    );

    commands.spawn((
        CollisionEffectSpawner,
        ParticleEffectBundle::new(effect).with_spawner(spawner),
    ));
}

pub fn spawn_collision_effect(
    mut spawner: Query<(&mut EffectSpawner, &mut Transform), With<CollisionEffectSpawner>>,
    mut ev_bullet_collided: EventReader<BulletCollided>,
) {
    let (mut spawner, mut transform) = spawner.single_mut();

    for ev in ev_bullet_collided.iter() {
        transform.translation = ev.position;
        spawner.reset();
    }
}

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
