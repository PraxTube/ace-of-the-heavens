use bevy::prelude::*;

use crate::{GameAssets, GameState};

#[derive(Component)]
struct MatchmakingScreen;
#[derive(Component)]
struct MatchmakingText;

const MESSAGE: &str = "WAITING FOR 1 OTHER PLAYER";

fn spawn_title_text(commands: &mut Commands, font: Handle<Font>) -> Entity {
    let text_style = TextStyle {
        font,
        font_size: 75.0,
        color: Color::WHITE,
    };
    let text_bundle = TextBundle::from_sections([TextSection::new(MESSAGE, text_style)]);
    commands.spawn((MatchmakingText, text_bundle)).id()
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
            MatchmakingScreen,
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
    let quit_text = spawn_quit_text(commands, font.clone());
    commands
        .entity(text_root_node)
        .push_children(&[title_text, quit_text]);
}

fn spawn_matchmaking_screen(mut commands: Commands, assets: Res<GameAssets>) {
    spawn_text(&mut commands, assets.font.clone());
}

fn despawn_matchmaking_screen(
    mut commands: Commands,
    game_over_screens: Query<Entity, With<MatchmakingScreen>>,
) {
    for screen_component in &game_over_screens {
        commands.entity(screen_component).despawn_recursive();
    }
}

fn animate_matchmaking_screen(
    mut query: Query<&mut Text, With<MatchmakingText>>,
    mut ticks: Local<f32>,
    mut forward: Local<bool>,
    time: Res<Time>,
) {
    let mut text = if let Ok(t) = query.get_single_mut() {
        t
    } else {
        error!("there are multiple or none matchmaking texts, there should be exactly one");
        return;
    };

    *ticks += time.delta_seconds();
    if *ticks < 0.2 {
        return;
    }
    *ticks = 0.0;

    let content = &text.sections[0].value;
    let new_text = if content.ends_with('-') {
        *forward = !*forward;
        if *forward {
            MESSAGE.to_string() + " /"
        } else {
            MESSAGE.to_string() + " \\"
        }
    } else if content.ends_with('/') {
        MESSAGE.to_string() + " -"
    } else if content.ends_with('\\') {
        MESSAGE.to_string() + " |"
    } else {
        *forward = !*forward;
        if *forward {
            MESSAGE.to_string() + " /"
        } else {
            MESSAGE.to_string() + " \\"
        }
    };

    text.sections[0].value = new_text.to_string();
}

pub struct MatchmakingUiPlugin;

impl Plugin for MatchmakingUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            animate_matchmaking_screen.run_if(in_state(GameState::Matchmaking)),
        )
        .add_systems(OnEnter(GameState::Matchmaking), spawn_matchmaking_screen)
        .add_systems(OnExit(GameState::Matchmaking), despawn_matchmaking_screen);
    }
}
