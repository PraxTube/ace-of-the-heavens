use bevy::prelude::*;

use super::ui::MAX_SCORE;
use crate::game_logic::Score;
use crate::player::player::{P1_COLOR, P2_COLOR};
use crate::GameAssets;

#[derive(Component)]
pub struct ScoreIcon {
    index: usize,
}

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

fn spawn_score_circle(
    commands: &mut Commands,
    texture: Handle<Image>,
    handle: usize,
    index: usize,
) -> Entity {
    let color = if handle == 0 { P1_COLOR } else { P2_COLOR };
    commands
        .spawn((
            ScoreIcon { index },
            ImageBundle {
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
            },
        ))
        .id()
}

pub fn spawn_scoreboard(mut commands: Commands, assets: Res<GameAssets>) {
    let texture = assets.score_empty.clone();
    let font = assets.font.clone();

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
                position_type: PositionType::Absolute,
                ..default()
            },
            ..default()
        })
        .id();

    let mut children: Vec<Entity> = Vec::new();

    let handle = 0;
    for i in 0..MAX_SCORE {
        children.push(spawn_score_circle(
            &mut commands,
            texture.clone(),
            handle,
            i,
        ));
    }

    children.push(spawn_text(&mut commands, font));

    let handle = 1;
    for i in 0..MAX_SCORE {
        let i = MAX_SCORE * 2 - 1 - i;
        children.push(spawn_score_circle(
            &mut commands,
            texture.clone(),
            handle,
            i,
        ));
    }
    commands.entity(root_node).push_children(&children);
}

pub fn update_scoreboard(
    score: Res<Score>,
    mut score_icons: Query<(&ScoreIcon, &mut UiImage)>,
    assets: Res<GameAssets>,
) {
    let mut score_mask = [false; MAX_SCORE * 2];
    for i in 0..score.0 {
        score_mask[i] = true;
    }

    for i in 0..score.1 {
        score_mask[i + MAX_SCORE] = true;
    }

    for (score_icon, mut ui_image) in &mut score_icons {
        if score_mask[score_icon.index] {
            ui_image.texture = assets.score_full.clone();
        } else {
            ui_image.texture = assets.score_empty.clone();
        }
    }
}
