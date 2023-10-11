use std::hash::{Hash, Hasher};
use std::time::Duration;

use bevy::prelude::*;
use bevy_ggrs::*;

use super::player::PLAYER_RADIUS;
use crate::debug::DebugTransform;
use crate::input;
use crate::map::map::outside_of_borders;
use crate::map::obstacle::{collision, Obstacle};
use crate::network::GgrsConfig;
use crate::player::player::Player;
use crate::GameAssets;

pub const BULLET_RADIUS: f32 = 1.0;
const BULLET_MOVE_SPEED: f32 = 350.0 / 60.0;
const DAMAGE: u32 = 100;
const BULLET_RELOAD_TIME: f32 = 0.1;
const FIRE_HEAT: u32 = 80;

const LEFT_WING_BULLET_SPAWN: Vec3 = Vec3::new(10.0, 20.0, 0.0);
const RIGHT_WING_BULLET_SPAWN: Vec3 = Vec3::new(10.0, -20.0, 0.0);

const ROCKET_RADIUS: f32 = 1.5;
const ROCKET_MOVE_SPEED: f32 = 700.0 / 60.0;
const ROCKET_RELOAD_TIME: f32 = 0.5;
const ROCKET_EXPLOSION_RADIUS: f32 = 100.0;

#[derive(Component, Reflect, Default)]
#[reflect(Hash)]
pub struct Bullet {
    current_speed: f32,
    pub damage: u32,
    pub handle: usize,
    pub disabled: bool,
}

impl Bullet {
    fn new(player_speed: f32, extra_damage: u32, handle: usize) -> Bullet {
        Bullet {
            current_speed: BULLET_MOVE_SPEED + player_speed,
            damage: DAMAGE + extra_damage,
            handle,
            disabled: false,
        }
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Hash)]
pub struct Rocket {
    current_speed: f32,
    pub handle: usize,
    pub disabled: bool,
}

impl Rocket {
    fn new(player_speed: f32, handle: usize) -> Rocket {
        Rocket {
            current_speed: ROCKET_MOVE_SPEED + player_speed,
            handle,
            disabled: false,
        }
    }
}

#[derive(Component)]
pub struct RocketExplosion(usize, bool, usize);

#[derive(Event)]
pub struct SpawnRocketExplosion(Vec3, usize);

impl Hash for Bullet {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.current_speed.to_bits().hash(state);
    }
}

impl Hash for Rocket {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.current_speed.to_bits().hash(state);
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Hash)]
pub struct BulletTimer {
    pub timer: Timer,
}

impl BulletTimer {
    pub fn default() -> BulletTimer {
        BulletTimer {
            timer: Timer::from_seconds(BULLET_RELOAD_TIME, TimerMode::Repeating),
        }
    }
}

impl Hash for BulletTimer {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.timer.duration().as_secs_f32().to_bits().hash(state);
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Hash)]
pub struct RocketTimer {
    pub timer: Timer,
}

impl RocketTimer {
    pub fn default() -> RocketTimer {
        RocketTimer {
            timer: Timer::from_seconds(ROCKET_RELOAD_TIME, TimerMode::Repeating),
        }
    }
}

impl Hash for RocketTimer {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.timer.duration().as_secs_f32().to_bits().hash(state);
    }
}

#[derive(Component, Reflect, Default)]
pub struct ExplosionAnimationTimer {
    pub timer: Timer,
}

impl ExplosionAnimationTimer {
    pub fn default() -> ExplosionAnimationTimer {
        ExplosionAnimationTimer {
            timer: Timer::from_seconds(0.075, TimerMode::Repeating),
        }
    }
}

fn spawn_bullet(
    commands: &mut Commands,
    player: &Player,
    player_transform: &Transform,
    texture: Handle<Image>,
    spawn_offset: Vec3,
) {
    let transform = Transform::from_translation(
        player_transform.translation + player_transform.rotation.mul_vec3(spawn_offset),
    )
    .with_rotation(player_transform.rotation);
    commands
        .spawn((
            Bullet::new(player.current_speed, player.speed_ratio(), player.handle),
            DebugTransform::new(&transform),
            SpriteBundle {
                transform,
                texture,
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
    assets: Res<GameAssets>,
    mut players: Query<(&Transform, &mut Player, &mut BulletTimer)>,
) {
    for (player_transform, mut player, mut bullet_timer) in &mut players {
        let (input, _) = inputs[player.handle];
        if !input::fire(input) || !bullet_timer.timer.finished() {
            continue;
        }
        if player.overheated {
            continue;
        }

        spawn_bullet(
            &mut commands,
            &player,
            player_transform,
            assets.bullet.clone(),
            LEFT_WING_BULLET_SPAWN,
        );
        spawn_bullet(
            &mut commands,
            &player,
            player_transform,
            assets.bullet.clone(),
            RIGHT_WING_BULLET_SPAWN,
        );

        player.heat += FIRE_HEAT;
        bullet_timer.timer.reset();
    }
}

pub fn move_bullets(mut bullets: Query<(&mut Transform, &Bullet, &mut DebugTransform)>) {
    for (mut transform, bullet, mut debug_transform) in &mut bullets {
        let direction = transform.local_x();
        transform.translation += direction * bullet.current_speed;
        debug_transform.update(&transform);
    }
}

pub fn destroy_bullets(
    mut commands: Commands,
    bullets: Query<(Entity, &Bullet, &Transform)>,
    obstacles: Query<&Obstacle, (Without<Player>, Without<Bullet>)>,
) {
    for (entity, bullet, transform) in &bullets {
        if bullet.disabled || outside_of_borders(transform.translation) {
            commands.entity(entity).despawn_recursive();
        }

        for obstacle in &obstacles {
            if collision(obstacle, transform.translation) {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

pub fn spawn_rocket_explosions(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut ev_spawn_rocket_explosion: EventReader<SpawnRocketExplosion>,
) {
    let texture = assets.explosion.clone();
    let texture_atlas = TextureAtlas::from_grid(texture, Vec2::new(32.0, 32.0), 8, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    for ev in ev_spawn_rocket_explosion.iter() {
        let transform = Transform::from_translation(ev.0).with_scale(Vec3::splat(4.0));
        commands
            .spawn((
                RocketExplosion(ev.1, false, 0),
                ExplosionAnimationTimer::default(),
                DebugTransform::new(&transform),
                SpriteSheetBundle {
                    transform,
                    texture_atlas: texture_atlas_handle.clone(),
                    ..default()
                },
            ))
            .add_rollback();
    }
}

fn spawn_rocket(
    commands: &mut Commands,
    player: &Player,
    player_transform: &Transform,
    texture: Handle<Image>,
    spawn_offset: Vec3,
) {
    let transform = Transform::from_translation(
        player_transform.translation + player_transform.rotation.mul_vec3(spawn_offset),
    )
    .with_rotation(player_transform.rotation);
    commands
        .spawn((
            Rocket::new(player.current_speed, player.handle),
            DebugTransform::new(&transform),
            SpriteBundle {
                transform,
                texture,
                ..default()
            },
        ))
        .add_rollback();
}

pub fn fire_rockets(
    mut commands: Commands,
    inputs: Res<PlayerInputs<GgrsConfig>>,
    assets: Res<GameAssets>,
    mut players: Query<(&Transform, &Player, &mut RocketTimer)>,
) {
    for (player_transform, player, mut rocket_timer) in &mut players {
        let (input, _) = inputs[player.handle];
        if !input::rocket(input) || !rocket_timer.timer.finished() {
            continue;
        }

        let texture = if player.handle == 0 {
            assets.rocket1.clone()
        } else {
            assets.rocket2.clone()
        };

        spawn_rocket(
            &mut commands,
            &player,
            player_transform,
            texture,
            Vec3::default(),
        );

        rocket_timer.timer.reset();
    }
}

pub fn move_rockets(mut rockets: Query<(&mut Transform, &Rocket, &mut DebugTransform)>) {
    for (mut transform, rocket, mut debug_transform) in &mut rockets {
        let direction = transform.local_x();
        transform.translation += direction * rocket.current_speed;
        debug_transform.update(&transform);
    }
}

pub fn destroy_rockets(
    mut commands: Commands,
    rockets: Query<(Entity, &Rocket, &Transform)>,
    obstacles: Query<&Obstacle, (Without<Player>, Without<Rocket>)>,
    players: Query<(&Transform, &Player)>,
    mut ev_spawn_rocket_explosion: EventWriter<SpawnRocketExplosion>,
) {
    for (entity, rocket, rocket_transform) in &rockets {
        if rocket.disabled || outside_of_borders(rocket_transform.translation) {
            ev_spawn_rocket_explosion.send(SpawnRocketExplosion(
                rocket_transform.translation,
                rocket.handle,
            ));
            commands.entity(entity).despawn_recursive();
            continue;
        }

        for obstacle in &obstacles {
            if collision(obstacle, rocket_transform.translation) {
                ev_spawn_rocket_explosion.send(SpawnRocketExplosion(
                    rocket_transform.translation,
                    rocket.handle,
                ));
                commands.entity(entity).despawn_recursive();
            }
        }

        for (player_transform, player) in &players {
            if player.handle == rocket.handle {
                continue;
            }

            let distance = Vec2::distance_squared(
                player_transform.translation.truncate(),
                rocket_transform.translation.truncate(),
            );
            if distance < PLAYER_RADIUS * PLAYER_RADIUS + ROCKET_RADIUS * ROCKET_RADIUS {
                ev_spawn_rocket_explosion.send(SpawnRocketExplosion(
                    rocket_transform.translation,
                    rocket.handle,
                ));
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

pub fn animate_rocket_explosions(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &mut ExplosionAnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (entity, mut timer, mut sprite) in &mut query {
        timer.timer.tick(Duration::from_secs_f32(1.0 / 60.0));
        if timer.timer.just_finished() {
            if sprite.index == 7 {
                commands.entity(entity).despawn_recursive();
                continue;
            }
            sprite.index += 1
        }
    }
}

pub fn check_explosion(
    mut explosions: Query<(&mut RocketExplosion, &Transform)>,
    mut players: Query<(&mut Player, &Transform)>,
) {
    for (mut rocket_explosion, rocket_transform) in &mut explosions {
        if rocket_explosion.2 > 1 {
            continue;
        }
        rocket_explosion.2 += 1;

        for (mut player, player_transform) in &mut players {
            if player.handle == rocket_explosion.0 {
                continue;
            }

            let distance = Vec2::distance_squared(
                player_transform.translation.truncate(),
                rocket_transform.translation.truncate(),
            );
            if distance
                < PLAYER_RADIUS * PLAYER_RADIUS + ROCKET_EXPLOSION_RADIUS * ROCKET_EXPLOSION_RADIUS
            {
                player.health = 0;
            }
        }
    }
}
