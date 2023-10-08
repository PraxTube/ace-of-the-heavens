use bevy::prelude::*;

use crate::game_logic::Score;
use crate::player::player::{P1_COLOR, P2_COLOR};
use crate::GameAssets;

use super::ui::MAX_SCORE;

#[derive(Component)]
pub struct RoundScreen;

#[derive(Component)]
pub struct RoundScore;

pub fn spawn_round_over_screen(mut commands: Commands, assets: Res<GameAssets>) {
    let root_node = commands
        .spawn((
            RoundScreen,
            NodeBundle {
                style: Style {
                    height: Val::Vh(100.0),
                    width: Val::Vw(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    position_type: PositionType::Absolute,
                    display: Display::None,
                    ..default()
                },
                z_index: ZIndex::Local(100),
                ..default()
            },
        ))
        .id();

    let background = commands
        .spawn((ImageBundle {
            style: Style {
                height: Val::Vh(100.0),
                width: Val::Vw(100.0),
                position_type: PositionType::Absolute,
                ..default()
            },
            image: UiImage {
                texture: assets.white_pixel.clone(),
                ..default()
            },
            background_color: BackgroundColor(Color::rgba(0.2, 0.2, 0.2, 0.65)),
            z_index: ZIndex::Local(-1),
            ..default()
        },))
        .id();

    let score = commands
        .spawn((
            RoundScore,
            ImageBundle {
                style: Style {
                    height: Val::Percent(15.0),
                    aspect_ratio: Some(1.0),
                    ..default()
                },
                image: UiImage {
                    texture: assets.score_full.clone(),
                    ..default()
                },
                ..default()
            },
        ))
        .id();

    commands
        .entity(root_node)
        .push_children(&[background, score]);
}

pub fn show_round_over_screen(
    mut round_screen: Query<&mut Style, With<RoundScreen>>,
    mut round_score: Query<&mut BackgroundColor, With<RoundScore>>,
    score: Res<Score>,
) {
    if score.0 == MAX_SCORE || score.1 == MAX_SCORE {
        return;
    }

    round_screen.single_mut().display = Display::Flex;
    *round_score.single_mut() = if score.2 == Some(0) {
        BackgroundColor(P1_COLOR)
    } else if score.2 == Some(1) {
        BackgroundColor(P2_COLOR)
    } else {
        BackgroundColor(Color::WHITE)
    };
}

pub fn hide_round_over_screen(mut round_screen: Query<&mut Style, With<RoundScreen>>) {
    round_screen.single_mut().display = Display::None;
}
