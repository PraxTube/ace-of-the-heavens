use std::time::Duration;

use bevy::core::FrameCount;
use bevy::prelude::*;
use bevy_ggrs::{AddRollbackCommandExtension, GgrsSchedule};

use crate::audio::RollbackSound;
use crate::player::spawning::despawn_players;
use crate::player::{LocalPlayerHandle, P1_COLOR, P2_COLOR};
use crate::world::{check_rematch, round_end_timeout};
use crate::{GameAssets, RollbackState};

#[derive(Resource, Reflect, Deref, DerefMut)]
#[reflect(Resource)]
pub struct RoundStartTimer(Timer);

impl Default for RoundStartTimer {
    fn default() -> Self {
        RoundStartTimer(Timer::from_seconds(1.5, TimerMode::Repeating))
    }
}

#[derive(Resource, Reflect, Deref, DerefMut)]
#[reflect(Resource)]
pub struct HideScreenTimer(Timer);

impl Default for HideScreenTimer {
    fn default() -> Self {
        HideScreenTimer(Timer::from_seconds(0.5, TimerMode::Repeating))
    }
}

#[derive(Component)]
struct RoundStartScreen;
#[derive(Component)]
pub struct RoundStartText;

fn spawn_round_start_screen(
    mut commands: Commands,
    assets: Res<GameAssets>,
    local_handle: Res<LocalPlayerHandle>,
) {
    let text_root_node = commands
        .spawn((
            RoundStartScreen,
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
                z_index: ZIndex::Local(101),
                ..default()
            },
        ))
        .add_rollback()
        .id();

    let text_style = TextStyle {
        font: assets.font.clone(),
        font_size: 100.0,
        color: Color::WHITE,
    };
    let text_bundle = TextBundle::from_sections([TextSection::new(
        "PREPARE".to_string(),
        TextStyle {
            color: if local_handle.0 == 0 {
                P1_COLOR
            } else {
                P2_COLOR
            },
            ..text_style.clone()
        },
    )]);

    let text_node = commands
        .spawn((RoundStartText, text_bundle))
        .add_rollback()
        .id();
    commands.entity(text_root_node).push_children(&[text_node]);
}

pub fn round_start_timeout(
    mut timer: ResMut<RoundStartTimer>,
    mut next_state: ResMut<NextState<RollbackState>>,
) {
    timer.tick(Duration::from_secs_f32(1.0 / 60.0));

    if timer.just_finished() {
        next_state.set(RollbackState::InRound);
    }
}

fn get_text_section(timer: &Res<RoundStartTimer>, text_section: &TextSection) -> TextSection {
    let time = (timer.duration() - timer.elapsed()).as_secs_f32();
    let (value, font_size) = if timer.just_finished() {
        ("GO!", 200.0)
    } else if time < timer.duration().as_secs_f32() / 3.0 {
        ("SET", 100.0)
    } else if time < timer.duration().as_secs_f32() * 2.0 / 3.0 {
        ("READY", 100.0)
    } else {
        ("PREPARE", 100.0)
    };
    TextSection {
        value: value.to_string(),
        style: TextStyle {
            font_size,
            ..text_section.style.clone()
        },
    }
}

pub fn animate_round_start_screen(
    timer: Res<RoundStartTimer>,
    mut text: Query<&mut Text, With<RoundStartText>>,
) {
    let mut text = text.single_mut();
    text.sections[0] = get_text_section(&timer, &text.sections[0]);
}

fn show_round_start_screen(
    mut commands: Commands,
    assets: Res<GameAssets>,
    frame: Res<FrameCount>,
    mut screen: Query<&mut Style, With<RoundStartScreen>>,
) {
    screen.single_mut().display = Display::Flex;
    commands.spawn(RollbackSound {
        clip: assets.round_start_sound.clone(),
        start_frame: frame.0 as usize,
        volume: 0.35,
        ..default()
    });
}

fn hide_round_start_screen(
    mut timer: ResMut<HideScreenTimer>,
    mut screen: Query<&mut Style, With<RoundStartScreen>>,
) {
    let mut screen = screen.single_mut();

    if screen.display == Display::None {
        return;
    }

    timer.tick(Duration::from_secs_f32(1.0 / 60.));

    if timer.just_finished() {
        screen.display = Display::None;
    }
}

pub struct RoundStartUiPlugin;

impl Plugin for RoundStartUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            GgrsSchedule,
            (
                animate_round_start_screen
                    .run_if(in_state(RollbackState::RoundStart))
                    .after(round_start_timeout),
                hide_round_start_screen.run_if(in_state(RollbackState::InRound)),
            )
                .chain()
                .after(apply_state_transition::<RollbackState>),
        )
        .add_systems(
            GgrsSchedule,
            round_start_timeout
                .ambiguous_with(round_end_timeout)
                .ambiguous_with(check_rematch)
                .ambiguous_with(despawn_players)
                .distributive_run_if(in_state(RollbackState::RoundStart))
                .after(apply_state_transition::<RollbackState>),
        )
        .add_systems(OnExit(RollbackState::Setup), spawn_round_start_screen)
        .add_systems(OnEnter(RollbackState::RoundStart), show_round_start_screen)
        .add_systems(OnExit(RollbackState::RoundStart), hide_round_start_screen);
    }
}
