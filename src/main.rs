use bevy::prelude::*;
use bevy_ggrs::*;

mod input;
mod network;

use network::GgrsConfig;

#[derive(Component)]
struct Player {
    handle: usize,
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins.set(ImagePlugin::default_nearest()),))
        .add_ggrs_plugin(
            GgrsPlugin::<GgrsConfig>::new()
                .with_input_system(input::input)
                .register_rollback_component::<Transform>(),
        )
        .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.5)))
        .add_systems(
            Startup,
            (setup, spawn_players, network::start_matchbox_socket),
        )
        .add_systems(Update, network::wait_for_players)
        .add_systems(GgrsSchedule, move_players)
        .run();
}

fn move_players(
    time: Res<Time>,
    inputs: Res<PlayerInputs<GgrsConfig>>,
    mut players: Query<(&mut Transform, &Player)>,
) {
    for (mut transform, player) in &mut players {
        let (input, _) = inputs[player.handle];

        let direction = input::direction(input);

        if direction == Vec2::ZERO {
            continue;
        }

        let direction = direction.normalize_or_zero().extend(0.0);
        let speed = 300.0;

        transform.translation += direction * speed * time.delta_seconds();
    }
}

fn spawn_players(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("plane1.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 1, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn((
            Player { handle: 0 },
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(0),
                transform: Transform::from_scale(Vec3::splat(5.0))
                    .with_translation(Vec3::new(-200.0, 0.0, 0.0)),
                ..default()
            },
        ))
        .add_rollback();

    let texture_handle = asset_server.load("plane2.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 1, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn((
            Player { handle: 1 },
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(0),
                transform: Transform::from_scale(Vec3::splat(5.0))
                    .with_translation(Vec3::new(200.0, 0.0, 0.0)),
                ..default()
            },
        ))
        .add_rollback();
}

fn setup(mut commands: Commands) {
    let camera_bundle = Camera2dBundle::default();
    commands.spawn(camera_bundle);
}
