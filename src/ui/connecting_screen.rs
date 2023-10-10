use std::time::Duration;

use bevy::prelude::*;

use crate::{
    game_logic::Seeds, network::ggrs_config::PLAYER_COUNT, GameAssets, GameState, RollbackState,
};

#[derive(Component)]
pub struct ConnectingScreen;
#[derive(Component)]
pub struct SeedStatus;
#[derive(Component)]
pub struct TimerBar;

#[derive(Resource, Reflect, Deref, DerefMut)]
#[reflect(Resource)]
pub struct ConnectingTimer(Timer);

impl Default for ConnectingTimer {
    fn default() -> Self {
        ConnectingTimer(Timer::from_seconds(3.0, TimerMode::Repeating))
    }
}

fn spawn_title_text(commands: &mut Commands, font: Handle<Font>) -> Entity {
    let text_style = TextStyle {
        font,
        font_size: 75.0,
        color: Color::WHITE,
    };
    let text_bundle =
        TextBundle::from_sections([TextSection::new("CONNECTING...".to_string(), text_style)])
            .with_text_alignment(TextAlignment::Center);
    commands.spawn(text_bundle).id()
}

fn spawn_seed_text(commands: &mut Commands, font: Handle<Font>) -> Entity {
    let text_style = TextStyle {
        font,
        font_size: 40.0,
        color: Color::WHITE,
    };
    let text_bundle = TextBundle::from_sections([TextSection::new(
        "FETCHING SEED FROM PEER...".to_string(),
        text_style,
    )])
    .with_text_alignment(TextAlignment::Center);
    commands.spawn((SeedStatus, text_bundle)).id()
}

fn spawn_text(commands: &mut Commands, font: Handle<Font>) {
    let text_root_node = commands
        .spawn((
            ConnectingScreen,
            NodeBundle {
                style: Style {
                    height: Val::Vh(100.0),
                    width: Val::Vw(100.0),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Vh(10.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    position_type: PositionType::Absolute,
                    ..default()
                },
                z_index: ZIndex::Local(101),
                ..default()
            },
        ))
        .id();
    let title_text = spawn_title_text(commands, font.clone());
    let seed_text = spawn_seed_text(commands, font.clone());
    commands
        .entity(text_root_node)
        .push_children(&[title_text, seed_text]);
}

fn spawn_timer_bar(commands: &mut Commands) {
    commands.spawn((
        TimerBar,
        ConnectingScreen,
        ImageBundle {
            style: Style {
                height: Val::Vh(10.0),
                width: Val::Vw(0.0),
                align_self: AlignSelf::End,
                position_type: PositionType::Absolute,
                ..default()
            },
            z_index: ZIndex::Local(101),
            ..default()
        },
    ));
}

pub fn spawn_connecting_screen(mut commands: Commands, assets: Res<GameAssets>) {
    spawn_text(&mut commands, assets.font.clone());
    spawn_timer_bar(&mut commands);
}

pub fn despawn_connecting_screen(
    mut commands: Commands,
    game_over_screens: Query<Entity, With<ConnectingScreen>>,
) {
    for screen_component in &game_over_screens {
        commands.entity(screen_component).despawn_recursive();
    }
}

pub fn tick_connecting_timer(
    mut timer: ResMut<ConnectingTimer>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_rollback_state: ResMut<NextState<RollbackState>>,
) {
    timer.tick(Duration::from_secs_f32(1.0 / 60.0));

    if timer.just_finished() {
        next_game_state.set(GameState::InGame);
        next_rollback_state.set(RollbackState::RoundStart);
    }
}

pub fn animate_connecting_screen(
    timer: Res<ConnectingTimer>,
    seeds: Res<Seeds>,
    mut seed_text: Query<&mut Text, With<SeedStatus>>,
    mut timer_bar: Query<&mut Style, With<TimerBar>>,
) {
    if seeds.0.len() == PLAYER_COUNT {
        seed_text.single_mut().sections[0].value = "RECEIVED SEED!".to_string();
    }

    let progress =
        (timer.elapsed().as_secs_f32() / timer.duration().as_secs_f32() * 100.0).clamp(0.0, 100.0);
    timer_bar.single_mut().width = Val::Vw(progress);
}
