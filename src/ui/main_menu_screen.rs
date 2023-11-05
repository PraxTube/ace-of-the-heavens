use bevy::prelude::*;

use super::MainMenuState;
use crate::{GameAssets, GameState};

#[derive(Component)]
struct MainMenuScreen;

fn spawn_title_text(commands: &mut Commands, font: Handle<Font>) -> Entity {
    let text_style = TextStyle {
        font,
        font_size: 75.0,
        color: Color::WHITE,
    };
    let text_bundle =
        TextBundle::from_sections([TextSection::new("MAIN MENU".to_string(), text_style)])
            .with_text_alignment(TextAlignment::Center);
    commands.spawn(text_bundle).id()
}

fn spawn_play_text(commands: &mut Commands, font: Handle<Font>) -> Entity {
    let text_style = TextStyle {
        font,
        font_size: 35.0,
        color: Color::WHITE,
    };
    let text_bundle =
        TextBundle::from_sections([TextSection::new("PRESS P TO PLAY".to_string(), text_style)]);
    commands.spawn(text_bundle).id()
}

fn spawn_help_text(commands: &mut Commands, font: Handle<Font>) -> Entity {
    let text_style = TextStyle {
        font,
        font_size: 35.0,
        color: Color::WHITE,
    };
    let text_bundle =
        TextBundle::from_sections([TextSection::new("PRESS H FOR HELP".to_string(), text_style)]);
    commands.spawn(text_bundle).id()
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

fn spawn_text(commands: &mut Commands, font: Handle<Font>) {
    let text_root_node = commands
        .spawn((
            MainMenuScreen,
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
    let play_text = spawn_play_text(commands, font.clone());
    let help_text = spawn_help_text(commands, font.clone());
    let quit_text = spawn_quit_text(commands, font.clone());
    commands
        .entity(text_root_node)
        .push_children(&[title_text, play_text, help_text, quit_text]);
}

fn spawn_main_menu_screen(mut commands: Commands, assets: Res<GameAssets>) {
    spawn_text(&mut commands, assets.font.clone());
}

fn despawn_main_menu_screen(
    mut commands: Commands,
    game_over_screens: Query<Entity, With<MainMenuScreen>>,
) {
    for screen_component in &game_over_screens {
        commands.entity(screen_component).despawn_recursive();
    }
}

fn play_game(
    keys: Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    gamepads: Res<Gamepads>,
    button_inputs: Res<Input<GamepadButton>>,
) {
    let mut pressed = keys.pressed(KeyCode::P);
    for gamepad in gamepads.iter() {
        if button_inputs.pressed(GamepadButton::new(gamepad, GamepadButtonType::South))
            || button_inputs.pressed(GamepadButton::new(gamepad, GamepadButtonType::North))
        {
            pressed = true;
        }
    }
    if pressed {
        next_state.set(GameState::Matchmaking);
    }
}

fn help_menu(keys: Res<Input<KeyCode>>, mut next_state: ResMut<NextState<MainMenuState>>) {
    if keys.pressed(KeyCode::H) {
        next_state.set(MainMenuState::HelpMenu);
    }
}

pub struct MainMenuUiPlugin;

impl Plugin for MainMenuUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                play_game.run_if(
                    in_state(GameState::MainMenu).and_then(in_state(MainMenuState::MainMenu)),
                ),
                help_menu.run_if(
                    in_state(GameState::MainMenu).and_then(in_state(MainMenuState::MainMenu)),
                ),
            ),
        )
        .add_systems(OnEnter(GameState::MainMenu), spawn_main_menu_screen)
        .add_systems(OnExit(GameState::MainMenu), despawn_main_menu_screen)
        .add_systems(OnExit(MainMenuState::HelpMenu), spawn_main_menu_screen)
        .add_systems(OnEnter(MainMenuState::HelpMenu), despawn_main_menu_screen);
    }
}
