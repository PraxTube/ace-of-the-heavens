use bevy::prelude::*;
use bevy_ggrs::AddRollbackCommandExtension;

use super::FadedLoopSound;
use crate::game_logic::Score;
use crate::ui::MAX_SCORE;
use crate::GameAssets;

#[derive(Component)]
pub struct BgmStage {
    stage: usize,
}

pub fn check_bgm_stage(
    mut commands: Commands,
    assets: Res<GameAssets>,
    score: Res<Score>,
    mut query: Query<(&mut FadedLoopSound, &BgmStage)>,
) {
    let match_point = score.p1 == MAX_SCORE - 1 || score.p2 == MAX_SCORE - 1;
    let (clip, stage) = if match_point {
        // Matchpoint Round
        (assets.bgm_match_point.clone(), 1)
    } else {
        // Normal Round
        (assets.bgm.clone(), 0)
    };

    let bgm_count = query.iter().count();
    if bgm_count == 0 {
        // Start Round
        commands
            .spawn((
                BgmStage { stage },
                FadedLoopSound {
                    clip,
                    volume: 0.075,
                    despawn_on_silence: true,
                    ..default()
                },
            ))
            .add_rollback();
        return;
    } else if bgm_count > 1 {
        panic!(
            "there are {} bgm's playing, only one or none should be playing",
            bgm_count
        );
    }
    let (mut sound, bgm_stage) = query.single_mut();

    // We are in a normal round and the correct BGM is already playing
    if !match_point && bgm_stage.stage == 0 {
        return;
    }
    // We are at match point and the correct BGM is already playing
    if match_point && bgm_stage.stage == 1 {
        return;
    }

    sound.should_play = false;
    commands
        .spawn((
            BgmStage { stage },
            FadedLoopSound {
                clip,
                volume: 0.1,
                despawn_on_silence: true,
                ..default()
            },
        ))
        .add_rollback();
}

pub fn fade_out_game_over_bgm(mut query: Query<&mut FadedLoopSound>) {
    if query.iter().count() != 1 {
        error!(
            "there should be exactly one BGM playing at this point.\nHowever there are {} playing",
            query.iter().count()
        );
        return;
    }

    for mut sound in &mut query {
        sound.fade_out = 3.5;
        sound.should_play = false;
    }
}
