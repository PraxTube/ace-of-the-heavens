use rand::{self, Rng};

use bevy::prelude::*;

use super::bullet::BulletFired;
use crate::{misc::DeadSprite, world::CollisionEntity, GameAssets};

const TRANSLATION_STRENGTH: f32 = 300.0;
const ROTATION_STRENGTH: f32 = 30.0;
const SPRITE_SIZE: Vec2 = Vec2::new(30.0, 10.0);

#[derive(Component)]
pub struct BulletCasing {
    timer: Timer,
    direction: Vec3,
    translation_strength: f32,
    rotation_strength: f32,
}

impl BulletCasing {
    fn new(direction: Vec3, translation_strength: f32, rotation_strength: f32) -> Self {
        Self {
            timer: Timer::from_seconds(1.5, TimerMode::Once),
            direction,
            translation_strength,
            rotation_strength,
        }
    }
}

pub fn spawn_bullet_casings(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut ev_bullet_fired: EventReader<BulletFired>,
) {
    let texture = assets.white_pixel.clone();
    let mut rng = rand::thread_rng();

    for ev in ev_bullet_fired.iter() {
        commands.spawn((
            DeadSprite,
            CollisionEntity::default(),
            BulletCasing::new(
                ev.direction,
                rng.gen_range(0.75..1.0),
                rng.gen_range(0.3..1.0),
            ),
            SpriteBundle {
                texture: texture.clone(),
                transform: Transform::from_translation(ev.position - Vec3::new(0.0, 0.0, 10.0))
                    .with_rotation(Quat::from_rotation_z(rng.gen_range(-3.0..3.0))),
                sprite: Sprite {
                    custom_size: Some(SPRITE_SIZE),
                    color: Color::rgb(0.75, 0.6, 0.2),
                    ..default()
                },
                ..default()
            },
        ));
    }
}

pub fn animate_bullet_casings(
    mut query: Query<(&mut Transform, &mut BulletCasing)>,
    time: Res<Time>,
) {
    for (mut transform, mut bullet_casing) in &mut query {
        let fall = (bullet_casing.timer.duration().as_secs_f32()
            - bullet_casing.timer.elapsed_secs())
            / bullet_casing.timer.duration().as_secs_f32();

        transform.translation += bullet_casing.direction
            * fall.powi(3)
            * bullet_casing.translation_strength
            * TRANSLATION_STRENGTH
            * time.delta_seconds();
        transform.rotate_z(
            bullet_casing.rotation_strength
                * ROTATION_STRENGTH
                * fall.powi(2)
                * time.delta_seconds(),
        );
        transform.scale = Vec3::ONE * 0.5 * (1.0 + fall.powi(2));

        bullet_casing.timer.tick(time.delta());
    }
}

pub fn despawn_bullet_casing_component(
    mut commands: Commands,
    query: Query<(Entity, &BulletCasing, &CollisionEntity)>,
) {
    for (entity, bullet_casing, collision_entity) in &query {
        if !bullet_casing.timer.finished() && !collision_entity.disabled {
            continue;
        }

        commands.entity(entity).remove::<BulletCasing>();
        commands.entity(entity).remove::<CollisionEntity>();
    }
}
