use bevy::prelude::*;

use crate::network::session_stats::SessionStats;
use crate::GameAssets;

#[derive(Component)]
pub struct StatsText;

fn spawn_text(commands: &mut Commands, font: Handle<Font>) -> Entity {
    commands
        .spawn((
            StatsText,
            TextBundle::from_section(
                String::new(),
                TextStyle {
                    font,
                    font_size: 30.0,
                    color: Color::WHITE,
                },
            )
            .with_text_alignment(TextAlignment::Center),
        ))
        .id()
}

pub fn spawn_stats_text(mut commands: Commands, assets: Res<GameAssets>, stats: Res<SessionStats>) {
    let font = assets.font.clone();
    let root_node = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Vw(100.0),
                height: Val::Vh(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                ..default()
            },
            ..default()
        })
        .id();

    let text = spawn_text(&mut commands, font);
    commands.entity(root_node).push_children(&[text]);
}

pub fn update_stats_text(mut text: Query<&mut Text, With<StatsText>>, stats: Res<SessionStats>) {
    let mut text = text.single_mut();
    text.sections[0].value = format!("Ping {}", stats.ping);
}
