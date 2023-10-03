use bevy::prelude::*;

use super::ui::MAX_SCORE;
use crate::player::player::{P1_COLOR, P2_COLOR};
use crate::Score;

pub fn spawn_game_over_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    score: Res<Score>,
) {
    let texture = asset_server.load("ui/white-pixel.png");
    let font = asset_server.load("fonts/PressStart2P.ttf");

    commands.spawn((ImageBundle {
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
    },));

    let text_root_node = commands
        .spawn(NodeBundle {
            style: Style {
                height: Val::Vh(100.0),
                width: Val::Vw(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                ..default()
            },
            z_index: ZIndex::Local(101),
            ..default()
        })
        .id();

    let text_style = TextStyle {
        font,
        font_size: 50.0,
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

    let text_node = commands.spawn(text_bundle).id();
    commands.entity(text_root_node).push_children(&[text_node]);
}
