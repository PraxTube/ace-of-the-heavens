use bevy::prelude::*;
use bevy_ggrs::{AddRollbackCommandExtension, GgrsSchedule};

use super::round_start_screen::animate_round_start_screen;
use crate::player::{check_rematch_state, LocalPlayerHandle, P1_COLOR, P2_COLOR};
use crate::world::MAX_SCORE;
use crate::world::{Rematch, Score};
use crate::{GameAssets, RollbackState};

#[derive(Component)]
struct GameOverScreen;

#[derive(Component)]
struct RematchText;

#[derive(Component)]
struct WinnerText;

fn spawn_background(commands: &mut Commands, texture: Handle<Image>) {
    commands
        .spawn((
            GameOverScreen,
            ImageBundle {
                style: Style {
                    height: Val::Vh(100.0),
                    width: Val::Vw(100.0),
                    position_type: PositionType::Absolute,
                    display: Display::None,
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
        ))
        .add_rollback();
}

fn spawn_winner_text(commands: &mut Commands, font: Handle<Font>) -> Entity {
    let text_style = TextStyle {
        font,
        font_size: 100.0,
        color: Color::WHITE,
    };
    let text_bundle = TextBundle::from_sections([
        TextSection::new(
            String::new(),
            TextStyle {
                ..text_style.clone()
            },
        ),
        TextSection::new("WON".to_string(), text_style.clone()),
    ]);
    commands
        .spawn((GameOverScreen, WinnerText, text_bundle))
        .add_rollback()
        .id()
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
    commands
        .spawn((RematchText, text_bundle))
        .add_rollback()
        .id()
}

fn spawn_quit_text(commands: &mut Commands, font: Handle<Font>) -> Entity {
    let text_style = TextStyle {
        font,
        font_size: 25.0,
        color: Color::WHITE,
    };
    let text_bundle =
        TextBundle::from_sections([TextSection::new("PRESS Q TO QUIT".to_string(), text_style)]);
    commands.spawn(text_bundle).add_rollback().id()
}

fn spawn_text(commands: &mut Commands, font: Handle<Font>) {
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
                    display: Display::None,
                    ..default()
                },
                z_index: ZIndex::Local(101),
                ..default()
            },
        ))
        .add_rollback()
        .id();
    let winner_text = spawn_winner_text(commands, font.clone());
    let rematch_text = spawn_rematch_text(commands, font.clone());
    let quit_text = spawn_quit_text(commands, font.clone());
    commands
        .entity(text_root_node)
        .push_children(&[winner_text, rematch_text, quit_text]);
}

fn spawn_game_over_screen(mut commands: Commands, assets: Res<GameAssets>) {
    spawn_background(&mut commands, assets.white_pixel.clone());
    spawn_text(&mut commands, assets.font.clone());
}

fn show_game_over_screen(mut screen_components: Query<&mut Style, With<GameOverScreen>>) {
    for mut screen_component in &mut screen_components {
        screen_component.display = Display::Flex;
    }
}

fn hide_game_over_screen(mut screen_components: Query<&mut Style, With<GameOverScreen>>) {
    for mut screen_component in &mut screen_components {
        screen_component.display = Display::None;
    }
}

fn update_winner_text(
    assets: Res<GameAssets>,
    mut winner_text: Query<&mut Text, With<WinnerText>>,
    score: Res<Score>,
) {
    let text_style = TextStyle {
        font: assets.font.clone(),
        font_size: 100.0,
        color: Color::WHITE,
    };
    winner_text.single_mut().sections[0] = if score.p1 == MAX_SCORE {
        TextSection::new(
            "ORANGE ".to_string(),
            TextStyle {
                color: P1_COLOR,
                ..text_style.clone()
            },
        )
    } else if score.p2 == MAX_SCORE {
        TextSection::new(
            "BLUE ".to_string(),
            TextStyle {
                color: P2_COLOR,
                ..text_style.clone()
            },
        )
    } else {
        TextSection::new("???".to_string(), text_style.clone())
    };
}

fn update_rematch_text(
    mut rematch_text: Query<&mut Text, With<RematchText>>,
    rematch: Res<Rematch>,
    local_handle: Res<LocalPlayerHandle>,
) {
    if rematch_text.iter().count() == 0 {
        return;
    }

    if (rematch.p1 && local_handle.0 == 0) || (rematch.p2 && local_handle.0 == 1) {
        let mut text = rematch_text.single_mut();
        text.sections[0].value = "SEND REQUEST".to_string();
    } else if (rematch.p1 && local_handle.0 != 0) || (rematch.p2 && local_handle.0 != 1) {
        let mut text = rematch_text.single_mut();
        text.sections[0].value = "PRESS R TO REMATCH\nENEMY WANTS REMATCH!".to_string();
    } else {
        let mut text = rematch_text.single_mut();
        text.sections[0].value = "PRESS R TO REMATCH".to_string();
    }
}

pub struct GameOverUiPlugin;

impl Plugin for GameOverUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(RollbackState::GameOver),
            (show_game_over_screen, update_winner_text),
        )
        .add_systems(OnExit(RollbackState::Setup), (spawn_game_over_screen,))
        .add_systems(OnExit(RollbackState::GameOver), hide_game_over_screen)
        .add_systems(
            GgrsSchedule,
            update_rematch_text
                .run_if(in_state(RollbackState::GameOver))
                .after(animate_round_start_screen)
                .after(check_rematch_state)
                .after(apply_state_transition::<RollbackState>),
        );
    }
}
