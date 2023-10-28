mod command;

use bevy::prelude::*;
use bevy_console::{ConsoleConfiguration, ConsolePlugin, ToggleConsoleKey};

use crate::{GameState, RollbackState};
use command::{apply_commands, receive_commands, reset_commands, AceCommandPlugin, AceCommands};

#[derive(Resource, Default)]
pub struct CommandQueue {
    pub queue: Vec<AceCommands>,
}

pub struct AceConsolePlugin;

impl Plugin for AceConsolePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            receive_commands
                .chain()
                .run_if(in_state(GameState::InRollbackGame)),
        )
        .add_plugins((ConsolePlugin, AceCommandPlugin))
        .insert_resource(ConsoleConfiguration {
            keys: vec![ToggleConsoleKey::KeyCode(KeyCode::F1)],
            ..default()
        })
        .init_resource::<CommandQueue>()
        .add_systems(OnEnter(RollbackState::InRound), apply_commands)
        .add_systems(OnEnter(GameState::InRollbackGame), reset_commands);
    }
}
