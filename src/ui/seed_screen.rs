use bevy::prelude::*;

use crate::game_logic::{determine_seed, Seeds};
use crate::GameAssets;

fn spawn_text(commands: &mut Commands, font: Handle<Font>, seed: u32) -> Entity {
    commands
        .spawn(
            TextBundle::from_section(
                format!("SEED: {}", seed),
                TextStyle {
                    font,
                    font_size: 15.0,
                    color: Color::WHITE,
                },
            )
            .with_text_alignment(TextAlignment::Center),
        )
        .id()
}

pub fn spawn_seed_screen(mut commands: Commands, assets: Res<GameAssets>, seeds: Res<Seeds>) {
    let font = assets.font.clone();

    // root node
    let root_node = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Vw(100.0),
                height: Val::Vh(98.5),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::End,
                position_type: PositionType::Absolute,
                ..default()
            },
            ..default()
        })
        .id();

    let text = spawn_text(&mut commands, font, determine_seed(&seeds));
    commands.entity(root_node).push_children(&[text]);
}
