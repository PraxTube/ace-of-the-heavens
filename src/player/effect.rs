use bevy::prelude::*;
use bevy_hanabi::prelude::*;

use super::player::{P1_COLOR, P2_COLOR};

use crate::player::health::PlayerTookDamage;

#[derive(Component)]
pub struct DamageEffectSpawner;

pub fn spawn_trail_effect(
    commands: &mut Commands,
    spawn_position: Vec3,
    spawn_rotation: Quat,
    effects: &mut ResMut<Assets<EffectAsset>>,
) -> Entity {
    let mut color_gradient = Gradient::new();
    color_gradient.add_key(0.0, Vec4::new(1.0, 1.0, 1.0, 1.0));
    color_gradient.add_key(1.0, Vec4::new(0.7, 0.7, 0.7, 0.0));
    let mut scale_gradient = Gradient::new();
    scale_gradient.add_key(0.0, Vec2::ONE * 5.0);
    scale_gradient.add_key(0.5, Vec2::ONE * 3.5);
    scale_gradient.add_key(1.0, Vec2::new(0.0, 0.0));

    let writer = ExprWriter::new();

    let age = writer.lit(0.0).uniform(writer.lit(0.3)).expr();
    let init_age = SetAttributeModifier::new(Attribute::AGE, age);

    let lifetime = writer.lit(0.75).uniform(writer.lit(1.0)).expr();
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    let init_pos = SetPositionSphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        radius: writer.lit(7.5).expr(),
        dimension: ShapeDimension::Volume,
    };

    let init_vel = SetVelocityCircleModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        axis: writer.lit(Vec3::Z).expr(),
        speed: writer.lit(0.0).expr(),
    };

    let spawner = Spawner::rate(50.0.into());
    let effect = effects.add(
        EffectAsset::new(50, spawner, writer.finish())
            .init(init_pos)
            .init(init_vel)
            .init(init_age)
            .init(init_lifetime)
            .render(SizeOverLifetimeModifier {
                gradient: scale_gradient,
                screen_space_size: false,
            })
            .render(ColorOverLifetimeModifier {
                gradient: color_gradient,
            }),
    );

    commands
        .spawn(ParticleEffectBundle {
            effect: ParticleEffect::new(effect),
            transform: Transform::from_translation(spawn_position).with_rotation(spawn_rotation),
            ..default()
        })
        .id()
}

pub fn spawn_damage_effect_spawner(
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
) {
    let mut scale_gradient = Gradient::new();
    scale_gradient.add_key(0.0, Vec2::ONE * 5.0);
    scale_gradient.add_key(0.5, Vec2::ONE * 3.5);
    scale_gradient.add_key(1.0, Vec2::new(0.0, 0.0));

    let spawner = Spawner::once(30.0.into(), false);

    let writer = ExprWriter::new();

    let color = writer.prop("spawn_color").expr();
    let init_color = SetAttributeModifier::new(Attribute::COLOR, color);

    // Give a bit of variation by randomizing the age per particle. This will
    // control the starting color and starting size of particles.
    let age = writer.lit(0.).uniform(writer.lit(0.2)).expr();
    let init_age = SetAttributeModifier::new(Attribute::AGE, age);

    // Give a bit of variation by randomizing the lifetime per particle
    let lifetime = writer.lit(0.8).uniform(writer.lit(1.2)).expr();
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    // Add constant downward acceleration to simulate gravity
    let accel = writer.lit(Vec3::Y * -8.).expr();
    let update_accel = AccelModifier::new(accel);

    // Add drag to make particles slow down a bit after the initial explosion
    let drag = writer.lit(1.).expr();
    let update_drag = LinearDragModifier::new(drag);

    let init_pos = SetPositionSphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        radius: writer.lit(2.).expr(),
        dimension: ShapeDimension::Volume,
    };

    // Give a bit of variation by randomizing the initial speed
    let init_vel = SetVelocitySphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        speed: (writer.rand(ScalarType::Float) * writer.lit(30.) + writer.lit(60.)).expr(),
    };

    let effect = effects.add(
        EffectAsset::new(1200, spawner, writer.finish())
            .with_property("spawn_color", 0xFFFFFFFFu32.into())
            .init(init_pos)
            .init(init_vel)
            .init(init_age)
            .init(init_lifetime)
            .init(init_color)
            .update(update_drag)
            .update(update_accel)
            .render(SizeOverLifetimeModifier {
                gradient: scale_gradient,
                screen_space_size: false,
            }),
    );

    commands
        .spawn((
            DamageEffectSpawner,
            ParticleEffectBundle::new(effect).with_spawner(spawner),
        ))
        .insert(Name::new("effect"));
}

fn color_to_u32(color: Color) -> u32 {
    let r = (color.r() * 255.0) as u8;
    let g = (color.g() * 255.0) as u8;
    let b = (color.b() * 255.0) as u8;
    let a = (color.a() * 255.0) as u8;

    (a as u32) << 24 | (b as u32) << 16 | (g as u32) << 8 | (r as u32)
}

pub fn spawn_damage_effect(
    mut ev_player_took_damage: EventReader<PlayerTookDamage>,
    mut spawner: Query<
        (
            &mut CompiledParticleEffect,
            &mut EffectSpawner,
            &mut Transform,
        ),
        With<DamageEffectSpawner>,
    >,
) {
    let Ok((mut effect, mut spawner, mut transform)) = spawner.get_single_mut() else {
        return;
    };

    for ev in ev_player_took_damage.iter() {
        let color = if ev.1 == 0 {
            color_to_u32(P1_COLOR)
        } else {
            color_to_u32(P2_COLOR)
        };
        transform.translation = ev.0.translation;
        effect.set_property("spawn_color", color.into());
        spawner.reset();
    }
}
