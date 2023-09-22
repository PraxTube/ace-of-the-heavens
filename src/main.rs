use bevy::prelude::*;
use bevy_ggrs::*;

mod environment;
mod input;
mod network;
mod player;

use network::GgrsConfig;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins.set(ImagePlugin::default_nearest()),))
        .add_ggrs_plugin(
            GgrsPlugin::<GgrsConfig>::new()
                .with_input_system(input::input)
                .register_rollback_component::<Transform>(),
        )
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(
            Startup,
            (
                setup,
                player::spawn_players,
                network::start_matchbox_socket,
                environment::spawn_background,
            ),
        )
        .add_systems(Update, network::wait_for_players)
        .add_systems(
            GgrsSchedule,
            (
                player::accelerate_players
                    .before(player::move_players)
                    .before(player::steer_players),
                player::steer_players.before(player::move_players),
                player::move_players,
            ),
        )
        .run();
}

fn setup(mut commands: Commands) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scale = 0.7;
    commands.spawn((camera_bundle,));
}
