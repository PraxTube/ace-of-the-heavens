use open;

use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;

use super::MainMenuState;
use crate::GameAssets;

const SCROLL_STRENGTH: f32 = 100.0;

#[derive(Component)]
pub struct HelpMenuScreen;

fn spawn_title_text(commands: &mut Commands, font: Handle<Font>) -> Entity {
    let text_style = TextStyle {
        font,
        font_size: 60.0,
        color: Color::WHITE,
    };
    let text_bundle =
        TextBundle::from_sections([TextSection::new("HELP MENU".to_string(), text_style)])
            .with_text_alignment(TextAlignment::Center);
    commands.spawn(text_bundle).id()
}

fn spawn_return_text(commands: &mut Commands, font: Handle<Font>) -> Entity {
    let text_style = TextStyle {
        font,
        font_size: 35.0,
        color: Color::WHITE,
    };
    let text_bundle = TextBundle::from_sections([TextSection::new(
        "PRESS R TO RETURN".to_string(),
        text_style,
    )]);
    commands.spawn(text_bundle).id()
}

fn spawn_open_text(commands: &mut Commands, font: Handle<Font>, text: String) -> Entity {
    let text_style = TextStyle {
        font,
        font_size: 25.0,
        color: Color::WHITE,
    };
    let mut text_bundle = TextBundle::from_sections([TextSection::new(text, text_style)]);
    text_bundle.style.width = Val::Vw(65.0);
    text_bundle.style.max_height = Val::Vh(35.0);
    commands.spawn(text_bundle).id()
}

fn spawn_quit_text(commands: &mut Commands, font: Handle<Font>) -> Entity {
    let text_style = TextStyle {
        font,
        font_size: 20.0,
        color: Color::WHITE,
    };
    let text_bundle =
        TextBundle::from_sections([TextSection::new("PRESS Q TO QUIT".to_string(), text_style)]);
    commands.spawn(text_bundle).id()
}

fn spawn_text(commands: &mut Commands, font: Handle<Font>, open_text: String) {
    let text_root_node = commands
        .spawn((
            HelpMenuScreen,
            NodeBundle {
                style: Style {
                    height: Val::Vh(100.0),
                    width: Val::Vw(100.0),
                    top: Val::Px(0.0),
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
    let return_text = spawn_return_text(commands, font.clone());
    let open_text = spawn_open_text(commands, font.clone(), open_text);
    let quit_text = spawn_quit_text(commands, font.clone());
    commands
        .entity(text_root_node)
        .push_children(&[title_text, return_text, open_text, quit_text]);
}

pub fn spawn_help_menu_screen(mut commands: Commands, assets: Res<GameAssets>) {
    let url = "http://rancic.org/aoth/help-menu/";
    let text = match open::that(url) {
        Ok(_) => "Opened webpage: ".to_string() + url,
        Err(err) => "ERROR, failed to open: ".to_string() + url + &format!("\n\n{}", err),
    };
    spawn_text(&mut commands, assets.font.clone(), text);
}

pub fn despawn_help_menu_screen(
    mut commands: Commands,
    game_over_screens: Query<Entity, With<HelpMenuScreen>>,
) {
    for screen_component in &game_over_screens {
        commands.entity(screen_component).despawn_recursive();
    }
}

pub fn return_to_main(keys: Res<Input<KeyCode>>, mut next_state: ResMut<NextState<MainMenuState>>) {
    if keys.pressed(KeyCode::R) {
        next_state.set(MainMenuState::MainMenu);
    }
}

pub fn scroll_help_screen(
    keys: Res<Input<KeyCode>>,
    mut ev_scroll: EventReader<MouseWheel>,
    mut query: Query<&mut Style, With<HelpMenuScreen>>,
) {
    let mut direction = 0.0;
    if keys.just_pressed(KeyCode::J) || keys.just_pressed(KeyCode::Down) {
        direction -= 1.0;
    }
    if keys.just_pressed(KeyCode::K) || keys.just_pressed(KeyCode::Up) {
        direction += 1.0;
    }

    for ev in ev_scroll.iter() {
        match ev.unit {
            MouseScrollUnit::Line => {
                direction += ev.y;
            }
            MouseScrollUnit::Pixel => {
                direction += ev.y;
            }
        }
    }

    if direction == 0.0 {
        return;
    }

    for mut style in &mut query {
        style.top = Val::try_add(&style.top, Val::Px(direction * SCROLL_STRENGTH)).unwrap();
    }
}
