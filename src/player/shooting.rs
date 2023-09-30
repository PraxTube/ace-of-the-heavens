use std::hash::{Hash, Hasher};

use bevy::prelude::*;
use bevy_ggrs::*;

use crate::debug::DebugTransform;
use crate::environment::outside_of_borders;
use crate::input;
use crate::network::GgrsConfig;
use crate::player::player::Player;
use crate::ImageAssets;

const MOVE_SPEED: f32 = 350.0 / 60.0;
pub const BULLET_RADIUS: f32 = 1.0;

const DAMAGE: u32 = 1;
const RELOAD_TIME: f32 = 0.1;
const OVERHEAT: u32 = 1000;
const HEAT_COOLDOWN: u32 = 12;
const HEAT_COOLDOWN_OVERHEAT: u32 = 5;
const FIRE_HEAT: u32 = 80;

const RELOAD_BAR_OFFSET: Vec3 = Vec3::new(0.0, 40.0, 0.0);
const RELOAD_BAR_SCALE: Vec3 = Vec3::new(50.0, 2.5, 1.0);

const LEFT_WING_BULLET_SPAWN: Vec3 = Vec3::new(10.0, 20.0, 0.0);
const RIGHT_WING_BULLET_SPAWN: Vec3 = Vec3::new(10.0, -20.0, 0.0);

#[derive(Component, Reflect, Default)]
#[reflect(Hash)]
pub struct Bullet {
    current_speed: f32,
    pub damage: u32,
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

impl Hash for Bullet {
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
            timer: Timer::from_seconds(RELOAD_TIME, TimerMode::Repeating),
        }
    }
}

impl Hash for BulletTimer {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.timer.duration().as_secs_f32().to_bits().hash(state);
    }
}

#[derive(Component)]
pub struct ReloadBar {
    pub handle: usize,
}

#[derive(Component)]
pub struct ReloadBarTicker;

pub fn reload_bullets(mut players: Query<&mut BulletTimer, With<Player>>) {
    for mut bullet_timer in &mut players {
        if !bullet_timer.timer.finished() {
            bullet_timer
                .timer
                // TODO use gloabal FPS from GGRSSchedule
                .tick(std::time::Duration::from_secs_f64(1.0 / 60.0));
        }
    }
}

pub fn cooldown_heat(mut players: Query<(&mut Player, &BulletTimer)>) {
    for (mut player, bullet_timer) in &mut players {
        if player.heat >= OVERHEAT {
            player.overheated = true;
        }

        if !bullet_timer.timer.finished() {
            continue;
        }

        if player.overheated {
            if player.heat <= HEAT_COOLDOWN_OVERHEAT {
                player.overheated = false;
                player.heat = 0;
            } else {
                player.heat -= HEAT_COOLDOWN_OVERHEAT;
            }
            continue;
        }

        player.heat = if player.heat <= HEAT_COOLDOWN {
            0
        } else {
            player.heat - HEAT_COOLDOWN
        };
    }
}

pub fn spawn_reload_bars(commands: &mut Commands, handle: usize) {
    let transform = Transform::from_scale(RELOAD_BAR_SCALE);
    let main = commands
        .spawn((
            ReloadBar { handle },
            DebugTransform::new(&transform),
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgba(1.0, 1.0, 1.0, 0.6),
                    custom_size: Some(Vec2::new(1.0, 1.0)),
                    ..default()
                },
                transform,
                ..default()
            },
        ))
        .add_rollback()
        .id();
    let transform = Transform::from_scale(Vec3::new(2.0 / RELOAD_BAR_SCALE.x, 6.0, 1.0));
    let ticker = commands
        .spawn((
            DebugTransform::new(&transform),
            ReloadBarTicker,
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgba(1.0, 1.0, 1.0, 0.9),
                    custom_size: Some(Vec2::new(1.0, 1.0)),
                    ..default()
                },
                transform,
                ..default()
            },
        ))
        .add_rollback()
        .id();
    commands.entity(main).push_children(&[ticker]);
}

pub fn update_reload_bars(
    mut reload_bars: Query<
        (&mut Transform, &ReloadBar, &Children, &mut DebugTransform),
        (Without<Player>, Without<ReloadBarTicker>),
    >,
    mut reload_bar_tickers: Query<
        (&mut Transform, &ReloadBarTicker, &mut DebugTransform),
        (Without<Player>, Without<ReloadBar>),
    >,
    players: Query<(&Transform, &Player), Without<ReloadBar>>,
) {
    for (player_transform, player) in &players {
        for (mut reload_bar_transform, reload_bar, children, mut reload_bar_debug_transform) in
            &mut reload_bars
        {
            if player.handle != reload_bar.handle {
                continue;
            }

            assert! { children.len() == 1 };

            reload_bar_transform.translation = player_transform.translation + RELOAD_BAR_OFFSET;
            reload_bar_debug_transform.update(&reload_bar_transform);

            let mut fill = reload_bar_tickers
                .get_mut(children[0])
                .expect("child of reloadbar (the ticker) is not accessable by it's parent");

            let x_fill = (100 * player.heat / OVERHEAT).clamp(0, 100);
            let x_fill = (x_fill as f32 / 100.0) - 0.5;

            fill.0.translation = Vec3::new(x_fill, fill.0.translation.y, fill.0.translation.z);
            fill.2.update(&fill.0);
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
    let transform = Transform::from_translation(
        player_transform.translation + player_transform.rotation.mul_vec3(spawn_offset),
    )
    .with_rotation(player_transform.rotation);
    commands
        .spawn((
            Bullet::new(player.current_speed, player.handle),
            DebugTransform::new(&transform),
            SpriteBundle {
                transform,
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
            &images,
            LEFT_WING_BULLET_SPAWN,
        );
        spawn_bullet(
            &mut commands,
            &player,
            player_transform,
            &images,
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

pub fn destroy_bullets(mut commands: Commands, bullets: Query<(Entity, &Bullet, &Transform)>) {
    for (entity, bullet, transform) in &bullets {
        if bullet.disabled || outside_of_borders(transform.translation) {
            commands.entity(entity).despawn_recursive();
        }
    }
}
