use bevy::prelude::*;
use bevy_ggrs::{AddRollbackCommandExtension, GgrsSchedule};

use crate::player::{P1_COLOR, P2_COLOR};
use crate::world::{check_rematch, Score, MAX_SCORE};
use crate::{GameAssets, RollbackState};

#[derive(Component)]
struct ScoreIcon {
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
        .add_rollback()
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
        .add_rollback()
        .id()
}

fn spawn_scoreboard(mut commands: Commands, assets: Res<GameAssets>) {
    let texture = assets.score_empty.clone();
    let font = assets.font.clone();

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
        .add_rollback()
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

fn update_scoreboard(
    score: Res<Score>,
    mut score_icons: Query<(&ScoreIcon, &mut UiImage)>,
    assets: Res<GameAssets>,
) {
    if !score.is_changed() {
        return;
    }

    let mut score_mask = [false; MAX_SCORE * 2];
    for score in score_mask.iter_mut().take(score.p1) {
        *score = true;
    }

    for i in 0..score.p2 {
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

pub struct ScoreboardUiPlugin;

impl Plugin for ScoreboardUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(RollbackState::Setup), spawn_scoreboard)
            .add_systems(
                GgrsSchedule,
                update_scoreboard
                    .run_if(not(in_state(RollbackState::Setup)))
                    .after(check_rematch)
                    .after(apply_state_transition::<RollbackState>),
            );
    }
}
