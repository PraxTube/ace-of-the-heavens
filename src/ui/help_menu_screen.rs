use open::that as open_url;

use bevy::prelude::*;

use super::MainMenuState;
use crate::{GameAssets, GameState};

#[derive(Component)]
struct HelpMenuScreen;

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

fn spawn_help_menu_screen(mut commands: Commands, assets: Res<GameAssets>) {
    let url = "http://rancic.org/aoth/help-menu/";
    let text = match open_url(url) {
        Ok(_) => "Opened webpage: ".to_string() + url,
        Err(err) => "ERROR, failed to open: ".to_string() + url + &format!("\n\n{}", err),
    };
    spawn_text(&mut commands, assets.font.clone(), text);
}

fn despawn_help_menu_screen(
    mut commands: Commands,
    game_over_screens: Query<Entity, With<HelpMenuScreen>>,
) {
    for screen_component in &game_over_screens {
        commands.entity(screen_component).despawn_recursive();
    }
}

fn return_to_main(keys: Res<Input<KeyCode>>, mut next_state: ResMut<NextState<MainMenuState>>) {
    if keys.pressed(KeyCode::R) {
        next_state.set(MainMenuState::MainMenu);
    }
}

pub struct HelpMenuUiPlugin;

impl Plugin for HelpMenuUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            return_to_main
                .run_if(in_state(GameState::MainMenu).and_then(in_state(MainMenuState::HelpMenu))),
        )
        .add_systems(OnEnter(MainMenuState::HelpMenu), spawn_help_menu_screen)
        .add_systems(OnExit(MainMenuState::HelpMenu), despawn_help_menu_screen);
    }
}
