use bevy::prelude::*;

use crate::{GameAssets, GameState};

#[derive(Component)]
pub struct MainMenuScreen;

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
    let quit_text = spawn_quit_text(commands, font.clone());
    commands
        .entity(text_root_node)
        .push_children(&[title_text, play_text, quit_text]);
}

pub fn spawn_main_menu_screen(mut commands: Commands, assets: Res<GameAssets>) {
    info!("spawn main menu");
    spawn_text(&mut commands, assets.font.clone());
}

pub fn despawn_main_menu_screen(
    mut commands: Commands,
    game_over_screens: Query<Entity, With<MainMenuScreen>>,
) {
    info!("despawn main menu");
    for screen_component in &game_over_screens {
        commands.entity(screen_component).despawn_recursive();
    }
}

pub fn play_game(keys: Res<Input<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
    if keys.pressed(KeyCode::P) {
        next_state.set(GameState::Matchmaking);
    }
}
