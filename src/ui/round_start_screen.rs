use std::time::Duration;

use bevy::prelude::*;

use crate::player::player::{LocalPlayerHandle, P1_COLOR, P2_COLOR};
use crate::RollbackState;

#[derive(Resource, Reflect, Deref, DerefMut)]
#[reflect(Resource)]
pub struct RoundStartTimer(Timer);

impl Default for RoundStartTimer {
    fn default() -> Self {
        RoundStartTimer(Timer::from_seconds(1.0, TimerMode::Repeating))
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
pub struct RoundStartScreen;
#[derive(Component)]
pub struct RoundStartText;

pub fn spawn_round_start_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    local_handle: Res<LocalPlayerHandle>,
) {
    let font = asset_server.load("fonts/PressStart2P.ttf");

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
                    ..default()
                },
                z_index: ZIndex::Local(101),
                ..default()
            },
        ))
        .id();

    let text_style = TextStyle {
        font,
        font_size: 100.0,
        color: Color::WHITE,
    };
    let text_bundle = TextBundle::from_sections([TextSection::new(
        "3".to_string(),
        TextStyle {
            color: if local_handle.0 == 0 {
                P1_COLOR
            } else {
                P2_COLOR
            },
            ..text_style.clone()
        },
    )]);

    let text_node = commands.spawn((RoundStartText, text_bundle)).id();
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

pub fn animate_round_start_screen(
    timer: Res<RoundStartTimer>,
    mut text: Query<&mut Text, With<RoundStartText>>,
) {
    let mut text = text.single_mut();

    if timer.just_finished() {
        text.sections[0].value = "GO!".to_string();
        return;
    }

    let time = 1.0 - timer.elapsed().as_secs_f32();

    let num = if time > 0.66 {
        "PREPARE"
    } else if time > 0.33 {
        "READY"
    } else {
        "SET"
    };

    text.sections[0].value = num.to_string();
}

pub fn show_round_start_screen(mut screen: Query<&mut Style, With<RoundStartScreen>>) {
    screen.single_mut().display = Display::Flex;
}

pub fn hide_round_start_screen(
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
