use crate::Score;
use bevy::prelude::*;

#[derive(Component)]
pub struct RoundScreen;

pub fn spawn_screen(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let white_pixel = asset_server.load("ui/white-pixel.png");
    let circle = asset_server.load("ui/score-empty.png");

    commands.spawn((
        RoundScreen,
        ImageBundle {
            style: Style {
                height: Val::Vh(100.0),
                width: Val::Vw(100.0),
                position_type: PositionType::Absolute,
                display: Display::None,
                ..default()
            },
            image: UiImage {
                texture: white_pixel,
                ..default()
            },
            background_color: BackgroundColor(Color::rgba(0.2, 0.2, 0.2, 0.65)),
            z_index: ZIndex::Local(100),
            ..default()
        },
    ));

    let circle_root_node = commands
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
                z_index: ZIndex::Local(101),
                ..default()
            },
        ))
        .id();

    let circle_node = commands
        .spawn((ImageBundle {
            style: Style {
                height: Val::Percent(15.0),
                aspect_ratio: Some(1.0),
                ..default()
            },
            image: UiImage {
                texture: circle,
                ..default()
            },
            ..default()
        },))
        .id();

    commands
        .entity(circle_root_node)
        .push_children(&[circle_node]);
}

pub fn show_round_screen(mut styles: Query<&mut Style, With<RoundScreen>>, score: Res<Score>) {
    for mut style in &mut styles {
        style.display = Display::Flex;
    }
}

pub fn hide_round_screen(mut styles: Query<&mut Style, With<RoundScreen>>) {
    for mut style in &mut styles {
        style.display = Display::None;
    }
}
