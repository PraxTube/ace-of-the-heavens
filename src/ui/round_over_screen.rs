use bevy::prelude::*;
use bevy_ggrs::AddRollbackCommandExtension;

use crate::player::{P1_COLOR, P2_COLOR};
use crate::world::MAX_SCORE;
use crate::world::{adjust_score, Score};
use crate::{GameAssets, RollbackState};

#[derive(Component)]
struct RoundScreen;

#[derive(Component)]
struct RoundScore;

fn spawn_round_over_screen(mut commands: Commands, assets: Res<GameAssets>) {
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
        .add_rollback()
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
        .add_rollback()
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
        .add_rollback()
        .id();

    commands
        .entity(root_node)
        .push_children(&[background, score]);
}

fn show_round_over_screen(
    mut round_screen: Query<&mut Style, With<RoundScreen>>,
    mut round_score: Query<&mut BackgroundColor, With<RoundScore>>,
    score: Res<Score>,
) {
    if score.p1 == MAX_SCORE || score.p2 == MAX_SCORE {
        return;
    }

    round_screen.single_mut().display = Display::Flex;
    *round_score.single_mut() = if score.last_winner == Some(0) {
        BackgroundColor(P1_COLOR)
    } else if score.last_winner == Some(1) {
        BackgroundColor(P2_COLOR)
    } else {
        BackgroundColor(Color::WHITE)
    };
}

fn hide_round_over_screen(mut round_screen: Query<&mut Style, With<RoundScreen>>) {
    round_screen.single_mut().display = Display::None;
}

pub struct RoundOverUiPlugin;

impl Plugin for RoundOverUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(RollbackState::Setup), spawn_round_over_screen)
            .add_systems(
                OnEnter(RollbackState::RoundEnd),
                show_round_over_screen.after(adjust_score),
            )
            .add_systems(OnExit(RollbackState::RoundEnd), hide_round_over_screen);
    }
}
