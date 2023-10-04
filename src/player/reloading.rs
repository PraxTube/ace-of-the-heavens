use bevy::prelude::*;
use bevy_ggrs::*;

use crate::debug::DebugTransform;
use crate::player::player::Player;
use crate::player::shooting::BulletTimer;

const OVERHEAT: u32 = 1000;
const HEAT_COOLDOWN_DELTA: u32 = 12;
const OVERHEAT_COOLDOWN_DELTA: u32 = 5;
const RELOAD_BAR_COLOR: Color = Color::rgba(0.9, 0.9, 0.9, 0.6);
const RELOAD_BAR_COLOR_OVERHEAT: Color = Color::rgba(
    0xFD as f32 / 255.0,
    0x38 as f32 / 255.0,
    0x38 as f32 / 255.0,
    0.6,
);
const RELOAD_BAR_TICKER_COLOR: Color = Color::rgba(1.0, 1.0, 1.0, 0.9);
const RELOAD_BAR_TICKER_COLOR_OVERHEAT: Color = Color::rgba(
    0xFD as f32 / 255.0,
    0x38 as f32 / 255.0,
    0x38 as f32 / 255.0,
    0.9,
);

const RELOAD_BAR_OFFSET: Vec3 = Vec3::new(0.0, 40.0, 10.0);
const RELOAD_BAR_SCALE: Vec3 = Vec3::new(50.0, 2.5, 1.0);

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
            if player.heat <= OVERHEAT_COOLDOWN_DELTA {
                player.overheated = false;
                player.heat = 0;
            } else {
                player.heat -= OVERHEAT_COOLDOWN_DELTA;
            }
            continue;
        }

        player.heat = if player.heat <= HEAT_COOLDOWN_DELTA {
            0
        } else {
            player.heat - HEAT_COOLDOWN_DELTA
        };
    }
}

pub fn color_reload_bars(
    mut reload_bars: Query<
        (&mut Sprite, &ReloadBar, &Children),
        (Without<Player>, Without<ReloadBarTicker>),
    >,
    mut reload_bar_tickers: Query<
        &mut Sprite,
        (Without<Player>, Without<ReloadBar>, With<ReloadBarTicker>),
    >,
    players: Query<&Player, Without<ReloadBar>>,
) {
    for player in &players {
        for (mut reload_bar_sprite, reload_bar, children) in &mut reload_bars {
            if player.handle != reload_bar.handle {
                continue;
            }

            assert! { children.len() == 1 };

            let mut reload_bar_ticker_sprite = reload_bar_tickers
                .get_mut(children[0])
                .expect("child of reloadbar (the ticker) is not accessable by it's parent");

            if player.overheated {
                reload_bar_sprite.color = RELOAD_BAR_COLOR_OVERHEAT;
                reload_bar_ticker_sprite.color = RELOAD_BAR_TICKER_COLOR_OVERHEAT;
            } else {
                reload_bar_sprite.color = RELOAD_BAR_COLOR;
                reload_bar_ticker_sprite.color = RELOAD_BAR_TICKER_COLOR;
            }
        }
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
                    color: RELOAD_BAR_COLOR,
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
                    color: RELOAD_BAR_TICKER_COLOR,
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
        (
            &mut Transform,
            &ReloadBar,
            &Children,
            &mut Visibility,
            &mut DebugTransform,
        ),
        (Without<Player>, Without<ReloadBarTicker>),
    >,
    mut reload_bar_tickers: Query<
        (&mut Transform, &ReloadBarTicker, &mut DebugTransform),
        (Without<Player>, Without<ReloadBar>),
    >,
    players: Query<(&Transform, &Player), Without<ReloadBar>>,
) {
    for (
        mut reload_bar_transform,
        reload_bar,
        children,
        mut visibility,
        mut reload_bar_debug_transform,
    ) in &mut reload_bars
    {
        *visibility = Visibility::Hidden;

        for (player_transform, player) in &players {
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

            *visibility = Visibility::Visible;
            break;
        }
    }
}