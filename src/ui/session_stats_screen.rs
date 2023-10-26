use bevy::prelude::*;

use crate::network::ggrs_config::PLAYER_COUNT;
use crate::network::session_stats::SessionStats;
use crate::GameAssets;

const ROW_GAP: f32 = 5.0;
const COLUMN_GAP: f32 = 0.0;

#[derive(Component)]
pub struct RootStatsScreen;

#[derive(Component)]
pub struct StatsText;

#[derive(Component)]
pub struct SessionStatsRow {
    pub handle: usize,
    pub sql: Entity,
    pub ping: Entity,
    pub kbps: Entity,
    pub lframes: Entity,
    pub rframes: Entity,
}

fn spawn_text(commands: &mut Commands, font: Handle<Font>, content: Option<&str>) -> Entity {
    let text = match content {
        Some(t) => t.to_string(),
        None => String::new(),
    };
    let sections = vec![TextSection {
        value: text,
        style: TextStyle {
            font,
            font_size: 30.0,
            color: Color::WHITE,
        },
    }];

    commands
        .spawn((
            StatsText,
            TextBundle {
                text: Text {
                    sections,
                    alignment: TextAlignment::Center,
                    ..default()
                },
                style: Style {
                    width: Val::Px(150.0),
                    ..default()
                },
                ..default()
            },
        ))
        .id()
}

fn spawn_session_header_row(commands: &mut Commands, font: Handle<Font>) -> Entity {
    let player = spawn_text(commands, font.clone(), Some("ID"));
    let sql = spawn_text(commands, font.clone(), Some("SQL"));
    let ping = spawn_text(commands, font.clone(), Some("PING"));
    let kbps = spawn_text(commands, font.clone(), Some("Kb/s"));
    let lframes = spawn_text(commands, font.clone(), Some("LF"));
    let rframes = spawn_text(commands, font.clone(), Some("RF"));

    let root_node = commands
        .spawn((NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Vh(COLUMN_GAP),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        },))
        .id();

    commands
        .entity(root_node)
        .push_children(&[player, sql, ping, kbps, lframes, rframes]);
    root_node
}

fn spawn_session_text_row(commands: &mut Commands, font: Handle<Font>, handle: usize) -> Entity {
    let player = spawn_text(commands, font.clone(), Some(&handle.to_string()));
    let sql = spawn_text(commands, font.clone(), None);
    let ping = spawn_text(commands, font.clone(), None);
    let kbps = spawn_text(commands, font.clone(), None);
    let lframes = spawn_text(commands, font.clone(), None);
    let rframes = spawn_text(commands, font.clone(), None);

    let root_node = commands
        .spawn((
            SessionStatsRow {
                handle,
                sql,
                ping,
                kbps,
                lframes,
                rframes,
            },
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Vh(COLUMN_GAP),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
        ))
        .id();

    commands
        .entity(root_node)
        .push_children(&[player, sql, ping, kbps, lframes, rframes]);
    root_node
}

pub fn spawn_stats_text(mut commands: Commands, assets: Res<GameAssets>) {
    let font = assets.font.clone();
    let root_node = commands
        .spawn((
            RootStatsScreen,
            NodeBundle {
                style: Style {
                    width: Val::Vw(100.0),
                    height: Val::Vh(100.0),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Vh(ROW_GAP),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    display: Display::None,
                    ..default()
                },
                ..default()
            },
        ))
        .id();

    let text = spawn_session_header_row(&mut commands, font.clone());
    commands.entity(root_node).push_children(&[text]);

    for handle in 0..PLAYER_COUNT {
        let text = spawn_session_text_row(&mut commands, font.clone(), handle);
        commands.entity(root_node).push_children(&[text]);
    }
}

// let mut text_str = String::new();
// // SQL, PING, Kb/s, LFrames, RFrames
// for (i, n_stats) in stats.network_stats.iter().enumerate() {
//     text_str += &format!("PLAYER {}, PING: {}\n", i, n_stats.ping);
// }
// text.sections[0].value = text_str;

pub fn update_stats_text(
    stats: Res<SessionStats>,
    row_stats: Query<&SessionStatsRow>,
    mut stats_text: Query<&mut Text, With<StatsText>>,
) {
    for session_stats_row in &row_stats {
        if let Ok(mut text) = stats_text.get_mut(session_stats_row.sql) {
            text.sections[0].value = stats.network_stats[session_stats_row.handle]
                .send_queue_len
                .to_string();
        }
        if let Ok(mut text) = stats_text.get_mut(session_stats_row.ping) {
            text.sections[0].value = stats.network_stats[session_stats_row.handle]
                .ping
                .to_string();
        }
        if let Ok(mut text) = stats_text.get_mut(session_stats_row.kbps) {
            text.sections[0].value = stats.network_stats[session_stats_row.handle]
                .kbps_sent
                .to_string();
        }
        if let Ok(mut text) = stats_text.get_mut(session_stats_row.lframes) {
            text.sections[0].value = stats.network_stats[session_stats_row.handle]
                .local_frames_behind
                .to_string();
        }
        if let Ok(mut text) = stats_text.get_mut(session_stats_row.rframes) {
            text.sections[0].value = stats.network_stats[session_stats_row.handle]
                .remote_frames_behind
                .to_string();
        }
    }
}

pub fn toggle_stats_visibility(
    keys: Res<Input<KeyCode>>,
    mut query: Query<&mut Style, With<RootStatsScreen>>,
) {
    if keys.just_pressed(KeyCode::Tab) {
        let mut style = query.single_mut();
        if style.display == Display::None {
            style.display = Display::Flex;
        } else {
            style.display = Display::None;
        }
    }
}
