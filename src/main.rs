use bevy::prelude::*;
use bevy_ggrs::*;
use bevy_matchbox::prelude::*;

mod input;

#[derive(Component)]
struct Player {
    handle: usize,
}

struct GgrsConfig;

impl ggrs::Config for GgrsConfig {
    type Input = u8;
    type State = u8;
    type Address = PeerId;
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
        .add_systems(Startup, (setup, spawn_players, start_matchbox_socket))
        .add_systems(Update, wait_for_players)
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

fn wait_for_players(mut commands: Commands, mut socket: ResMut<MatchboxSocket<SingleChannel>>) {
    if socket.get_channel(0).is_err() {
        return;
    }

    socket.update_peers();
    let players = socket.players();

    let num_players = 2;
    if players.len() < num_players {
        return;
    }

    info!("all peers have joined, going in-game");

    let mut session_builder = ggrs::SessionBuilder::<GgrsConfig>::new()
        .with_num_players(num_players)
        .with_input_delay(2);

    for (i, player) in players.into_iter().enumerate() {
        session_builder = session_builder
            .add_player(player, i)
            .expect("failed to add player");
    }

    let channel = socket.take_channel(0).unwrap();

    let ggrs_session = session_builder
        .start_p2p_session(channel)
        .expect("failed to start session");

    commands.insert_resource(bevy_ggrs::Session::P2P(ggrs_session));
}

fn start_matchbox_socket(mut commands: Commands) {
    let room_url = "ws://localhost:3536/";
    info!("connection to matchbox server: {}", room_url);
    commands.insert_resource(MatchboxSocket::new_ggrs(room_url));
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
