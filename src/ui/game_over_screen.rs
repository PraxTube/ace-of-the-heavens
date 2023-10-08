use bevy::prelude::*;

use super::ui::MAX_SCORE;
use crate::game_logic::{Rematch, Score};
use crate::player::player::{LocalPlayerHandle, P1_COLOR, P2_COLOR};
use crate::GameAssets;

#[derive(Component)]
pub struct GameOverScreen;

#[derive(Component)]
pub struct RematchText;

fn spawn_background(commands: &mut Commands, texture: Handle<Image>) {
    commands.spawn((
        GameOverScreen,
        ImageBundle {
            style: Style {
                height: Val::Vh(100.0),
                width: Val::Vw(100.0),
                position_type: PositionType::Absolute,
                ..default()
            },
            image: UiImage {
                texture,
                ..default()
            },
            background_color: BackgroundColor(Color::rgba(0.2, 0.2, 0.2, 0.85)),
            z_index: ZIndex::Local(100),
            ..default()
        },
    ));
}

fn spawn_winner_text(commands: &mut Commands, font: Handle<Font>, score: Res<Score>) -> Entity {
    let text_style = TextStyle {
        font,
        font_size: 100.0,
        color: Color::WHITE,
    };
    let text_bundle = if score.0 == MAX_SCORE {
        TextBundle::from_sections([
            TextSection::new(
                "ORANGE ".to_string(),
                TextStyle {
                    color: P1_COLOR,
                    ..text_style.clone()
                },
            ),
            TextSection::new("WON".to_string(), text_style.clone()),
        ])
    } else {
        TextBundle::from_sections([
            TextSection::new(
                "BLUE ".to_string(),
                TextStyle {
                    color: P2_COLOR,
                    ..text_style.clone()
                },
            ),
            TextSection::new("WON".to_string(), text_style.clone()),
        ])
    };
    commands.spawn((GameOverScreen, text_bundle)).id()
}

fn spawn_rematch_text(commands: &mut Commands, font: Handle<Font>) -> Entity {
    let text_style = TextStyle {
        font,
        font_size: 50.0,
        color: Color::WHITE,
    };
    let text_bundle = TextBundle::from_sections([TextSection::new(
        "PRESS R TO REMATCH".to_string(),
        text_style,
    )])
    .with_text_alignment(TextAlignment::Center);
    commands.spawn((RematchText, text_bundle)).id()
}

fn spawn_quit_text(commands: &mut Commands, font: Handle<Font>) -> Entity {
    let text_style = TextStyle {
        font,
        font_size: 25.0,
        color: Color::WHITE,
    };
    let text_bundle =
        TextBundle::from_sections([TextSection::new("PRESS Q TO QUIT".to_string(), text_style)]);
    commands.spawn(text_bundle).id()
}

fn spawn_text(commands: &mut Commands, font: Handle<Font>, score: Res<Score>) {
    let text_root_node = commands
        .spawn((
            GameOverScreen,
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
    let winner_text = spawn_winner_text(commands, font.clone(), score);
    let rematch_text = spawn_rematch_text(commands, font.clone());
    let quit_text = spawn_quit_text(commands, font.clone());
    commands
        .entity(text_root_node)
        .push_children(&[winner_text, rematch_text, quit_text]);
}

pub fn spawn_game_over_screen(mut commands: Commands, assets: Res<GameAssets>, score: Res<Score>) {
    spawn_background(&mut commands, assets.white_pixel.clone());
    spawn_text(&mut commands, assets.font.clone(), score);
}

pub fn despawn_game_over_screen(
    mut commands: Commands,
    game_over_screens: Query<Entity, With<GameOverScreen>>,
) {
    for screen_component in &game_over_screens {
        commands.entity(screen_component).despawn_recursive();
    }
}

pub fn update_rematch_text(
    mut rematch_text: Query<&mut Text, With<RematchText>>,
    rematch: Res<Rematch>,
    local_handle: Res<LocalPlayerHandle>,
) {
    if rematch_text.iter().count() == 0 {
        return;
    }

    if (rematch.0 && local_handle.0 == 0) || (rematch.1 && local_handle.0 == 1) {
        let mut text = rematch_text.single_mut();
        text.sections[0].value = "SEND REQUEST".to_string();
    } else if (rematch.0 && local_handle.0 != 0) || (rematch.1 && local_handle.0 != 1) {
        let mut text = rematch_text.single_mut();
        text.sections[0].value = "PRESS R TO REMATCH\nENEMY WANTS REMATCH!".to_string();
    } else {
        let mut text = rematch_text.single_mut();
        text.sections[0].value = "PRESS R TO REMATCH".to_string();
    }
}
