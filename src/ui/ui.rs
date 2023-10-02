use bevy::prelude::*;

use crate::player::player::{P1_COLOR, P2_COLOR};

fn spawn_text(commands: &mut Commands, font: Handle<Font>) -> Entity {
    commands
        .spawn(
            TextBundle::from_section(
                "- SCORE -",
                TextStyle {
                    font,
                    font_size: 50.0,
                    color: Color::WHITE,
                },
            )
            .with_text_alignment(TextAlignment::Center),
        )
        .id()
}

fn spawn_score_circle(commands: &mut Commands, texture: Handle<Image>, color: Color) -> Entity {
    commands
        .spawn(ImageBundle {
            style: Style {
                height: Val::Percent(40.0),
                aspect_ratio: Some(1.0),
                ..default()
            },
            image: UiImage {
                texture,
                ..default()
            },
            background_color: BackgroundColor(color),
            ..default()
        })
        .id()
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load("ui/score-empty.png");

    // root node
    let root_node = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Vw(100.0),
                height: Val::Vh(15.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                column_gap: Val::Vw(2.0),
                ..default()
            },
            ..default()
        })
        .id();

    let mut children: Vec<Entity> = Vec::new();

    for _ in 0..5 {
        children.push(spawn_score_circle(&mut commands, texture.clone(), P1_COLOR));
    }

    children.push(spawn_text(
        &mut commands,
        asset_server.load("fonts/PressStart2P.ttf"),
    ));

    for _ in 0..5 {
        children.push(spawn_score_circle(&mut commands, texture.clone(), P2_COLOR));
    }

    commands.entity(root_node).push_children(&children);
}
