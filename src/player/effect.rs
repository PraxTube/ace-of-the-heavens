use std::time::Duration;

use bevy::{core::FrameCount, prelude::*};
use bevy_ggrs::AddRollbackCommandExtension;
use bevy_hanabi::prelude::*;

use super::{Player, P1_COLOR, P2_COLOR};

use crate::{
    audio::RollbackSound, network::ggrs_config::GGRS_FPS, player::health::PlayerTookDamage,
    GameAssets,
};

const TRAIL_OFFSET_LEFT: Vec3 = Vec3::new(0.0, 30.0, -1.0);
const TRAIL_OFFSET_RIGHT: Vec3 = Vec3::new(0.0, -30.0, -1.0);

#[derive(Component)]
pub struct Trail;

#[derive(Component)]
pub struct KillTimer(Timer);

impl Default for KillTimer {
    fn default() -> Self {
        let mut timer = Timer::new(Duration::from_secs_f32(2.0), TimerMode::Once);
        timer.pause();
        Self(timer)
    }
}

#[derive(Component)]
pub struct DamageEffectSpawner;

pub fn spawn_trail_effect(
    commands: &mut Commands,
    effects: &mut ResMut<Assets<EffectAsset>>,
    offset: Vec3,
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

    let spawner = Spawner::rate(50.0.into()).with_starts_active(true);
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
        .spawn((
            Trail,
            KillTimer::default(),
            ParticleEffectBundle {
                effect: ParticleEffect::new(effect),
                transform: Transform::from_translation(offset),
                ..default()
            },
        ))
        .id()
}

pub fn spawn_player_trails(
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
    players: Query<Entity, With<Player>>,
) {
    for entity in &players {
        for offset in [TRAIL_OFFSET_LEFT, TRAIL_OFFSET_RIGHT] {
            let trail_effect = spawn_trail_effect(&mut commands, &mut effects, offset);
            commands.entity(entity).push_children(&[trail_effect]);
        }
    }
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
    mut commands: Commands,
    assets: Res<GameAssets>,
    frame: Res<FrameCount>,
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
    let (mut effect, mut spawner, mut transform) = spawner.single_mut();

    for ev in ev_player_took_damage.iter() {
        let color = if ev.1 == 0 {
            color_to_u32(P1_COLOR)
        } else {
            color_to_u32(P2_COLOR)
        };
        transform.translation = ev.0.translation;
        effect.set_property("spawn_color", color.into());
        spawner.reset();

        commands
            .spawn(RollbackSound {
                clip: assets.damage_sound.clone(),
                start_frame: frame.0 as usize,
                sub_key: frame.0 as usize,
                ..default()
            })
            .add_rollback();
    }
}

pub fn disable_trails(
    mut trails: Query<(&Parent, &mut EffectSpawner, &mut KillTimer), With<Trail>>,
    parents: Query<&Transform>,
) {
    for (parent, mut trail, mut kill_timer) in &mut trails {
        if parents.get(parent.get()).is_ok() {
            continue;
        }
        if !trail.is_active() {
            continue;
        }

        info!("disabling rocket",);
        trail.set_active(false);
        kill_timer.0.unpause();
    }
}

pub fn despawn_trails(
    mut commands: Commands,
    mut query: Query<(Entity, &mut KillTimer), With<Trail>>,
) {
    for (entity, mut kill_timer) in &mut query {
        kill_timer
            .0
            .tick(Duration::from_secs_f32(1.0 / GGRS_FPS as f32));
        if kill_timer.0.finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
