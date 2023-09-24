use bevy::prelude::*;
use bevy_ggrs::*;

use crate::input;
use crate::network::GgrsConfig;
use crate::player::player::Player;
use crate::ImageAssets;

const MOVE_SPEED: f32 = 300.0;
const DAMAGE: f32 = 1.0;
pub const BULLET_RADIUS: f32 = 1.0;

#[derive(Component)]
pub struct Bullet {
    current_speed: f32,
    pub damage: f32,
    pub handle: usize,
    pub disabled: bool,
}

#[derive(Component, Reflect, Default)]
pub struct BulletReady(pub bool);

impl Bullet {
    fn new(extra_speed: f32, handle: usize) -> Bullet {
        Bullet {
            current_speed: MOVE_SPEED + extra_speed,
            damage: DAMAGE,
            handle,
            disabled: false,
        }
    }
}

pub fn reload_bullets(
    inputs: Res<PlayerInputs<GgrsConfig>>,
    mut players: Query<(&mut BulletReady, &Player)>,
) {
    for (mut bullet_ready, player) in &mut players {
        let (input, _) = inputs[player.handle];
        if !input::fire(input) {
            bullet_ready.0 = true;
        }
    }
}

pub fn fire_bullets(
    mut commands: Commands,
    inputs: Res<PlayerInputs<GgrsConfig>>,
    images: Res<ImageAssets>,
    mut players: Query<(&Transform, &Player, &mut BulletReady)>,
) {
    for (transform, player, mut bullet_ready) in &mut players {
        let (input, _) = inputs[player.handle];
        if !input::fire(input) || !bullet_ready.0 {
            continue;
        }

        commands
            .spawn((
                Bullet::new(player.current_speed, player.handle),
                SpriteBundle {
                    transform: Transform::from_translation(transform.translation)
                        .with_rotation(transform.rotation),
                    texture: images.bullet.clone(),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(10.0, 3.0)),
                        ..default()
                    },
                    ..default()
                },
            ))
            .add_rollback();
        bullet_ready.0 = false;
    }
}

pub fn move_bullets(time: Res<Time>, mut bullets: Query<(&mut Transform, &Bullet)>) {
    for (mut transform, bullet) in &mut bullets {
        let direction = transform.local_x();
        transform.translation += direction * bullet.current_speed * time.delta_seconds();
    }
}

pub fn destroy_bullets(mut commands: Commands, bullets: Query<(Entity, &Bullet)>) {
    for (entity, bullet) in &bullets {
        if bullet.disabled {
            commands.entity(entity).despawn_recursive();
        }
    }
}
