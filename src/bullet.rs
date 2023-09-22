use bevy::prelude::*;
use bevy_ggrs::*;

use crate::input;
use crate::network::GgrsConfig;
use crate::player;
use crate::ImageAssets;

const MOVE_SPEED: f32 = 300.0;

#[derive(Component)]
pub struct Bullet {
    current_speed: f32,
}

impl Bullet {
    fn new(extra_speed: f32) -> Bullet {
        Bullet {
            current_speed: MOVE_SPEED + extra_speed,
        }
    }
}

pub fn fire_bullets(
    mut commands: Commands,
    inputs: Res<PlayerInputs<GgrsConfig>>,
    images: Res<ImageAssets>,
    mut players: Query<(&Transform, &player::Player, &mut player::BulletReady)>,
) {
    for (transform, player, mut bullet_ready) in &mut players {
        let (input, _) = inputs[player.handle];
        if !input::fire(input) || !bullet_ready.0 {
            continue;
        }

        commands
            .spawn((
                Bullet::new(player.current_speed),
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
