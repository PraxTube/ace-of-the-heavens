use bevy::prelude::*;
use bevy_ggrs::*;

use crate::environment::outside_of_borders;
use crate::input;
use crate::network::GgrsConfig;
use crate::player::player::Player;
use crate::ImageAssets;

const MOVE_SPEED: f32 = 350.0 / 60.0;
const DAMAGE: f32 = 1.0;
pub const BULLET_RADIUS: f32 = 1.0;
const RELOAD_TIME: f32 = 0.1;

const LEFT_WING_BULLET_SPAWN: Vec3 = Vec3::new(10.0, 20.0, 0.0);
const RIGHT_WING_BULLET_SPAWN: Vec3 = Vec3::new(10.0, -20.0, 0.0);

#[derive(Component, Reflect, Default)]
pub struct Bullet {
    current_speed: f32,
    pub damage: f32,
    pub handle: usize,
    pub disabled: bool,
}

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

#[derive(Component, Reflect, Default)]
pub struct BulletTimer {
    timer: Timer,
}

impl BulletTimer {
    pub fn default() -> BulletTimer {
        BulletTimer {
            timer: Timer::from_seconds(RELOAD_TIME, TimerMode::Repeating),
        }
    }
}

pub fn reload_bullets(time: Res<Time>, mut players: Query<&mut BulletTimer, With<Player>>) {
    for mut bullet_timer in &mut players {
        if !bullet_timer.timer.finished() {
            bullet_timer.timer.tick(time.delta());
        }
    }
}

fn spawn_bullet(
    commands: &mut Commands,
    player: &Player,
    player_transform: &Transform,
    images: &Res<ImageAssets>,
    spawn_offset: Vec3,
) {
    commands
        .spawn((
            Bullet::new(player.current_speed, player.handle),
            SpriteBundle {
                transform: Transform::from_translation(
                    player_transform.translation + player_transform.rotation.mul_vec3(spawn_offset),
                )
                .with_rotation(player_transform.rotation),
                texture: images.bullet.clone(),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(10.0, 3.0)),
                    ..default()
                },
                ..default()
            },
        ))
        .add_rollback();
}

pub fn fire_bullets(
    mut commands: Commands,
    inputs: Res<PlayerInputs<GgrsConfig>>,
    images: Res<ImageAssets>,
    mut players: Query<(&Transform, &Player, &mut BulletTimer)>,
) {
    for (player_transform, player, mut bullet_timer) in &mut players {
        let (input, _) = inputs[player.handle];
        if !input::fire(input) || !bullet_timer.timer.finished() {
            continue;
        }

        spawn_bullet(
            &mut commands,
            player,
            player_transform,
            &images,
            LEFT_WING_BULLET_SPAWN,
        );
        spawn_bullet(
            &mut commands,
            player,
            player_transform,
            &images,
            RIGHT_WING_BULLET_SPAWN,
        );

        bullet_timer.timer.reset();
    }
}

pub fn move_bullets(mut bullets: Query<(&mut Transform, &Bullet)>) {
    for (mut transform, bullet) in &mut bullets {
        let direction = transform.local_x();
        transform.translation += direction * bullet.current_speed;
    }
}

pub fn destroy_bullets(mut commands: Commands, bullets: Query<(Entity, &Bullet, &Transform)>) {
    for (entity, bullet, transform) in &bullets {
        if bullet.disabled || outside_of_borders(transform.translation) {
            commands.entity(entity).despawn_recursive();
        }
    }
}
