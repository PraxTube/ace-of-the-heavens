use std::time::Duration;

use bevy::prelude::*;
use bevy_ggrs::AddRollbackCommandExtension;
use bevy_hanabi::prelude::*;

use crate::network::ggrs_config::GGRS_FPS;

const LEFT_TRAIL_OFFSET: Vec3 = Vec3::new(0.0, 30.0, -1.0);
const RIGHT_TRAIL_OFFSET: Vec3 = Vec3::new(0.0, -30.0, -1.0);

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
        .add_rollback()
        .id()
}

pub fn spawn_player_trails(
    commands: &mut Commands,
    effects: &mut ResMut<Assets<EffectAsset>>,
    player: Entity,
) {
    for offset in [LEFT_TRAIL_OFFSET, RIGHT_TRAIL_OFFSET] {
        let trail_effect = spawn_trail_effect(commands, effects, offset);
        commands.entity(player).push_children(&[trail_effect]);
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
