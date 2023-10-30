use bevy::prelude::*;
use bevy_ggrs::Session;

use super::{RoundEndTimer, RoundStats, Score, Seeds};
use crate::audio::{BgmStage, PlaybackStates};
use crate::map;
use crate::network::session::Ready;
use crate::network::socket::AceSocket;
use crate::network::GgrsConfig;
use crate::player::{self, LocalPlayerHandle};
use crate::ui::round_start_screen::{HideScreenTimer, RoundStartTimer};
use crate::{GameState, RollbackState};

fn clear_world(
    mut commands: Commands,
    players: Query<Entity, With<player::Player>>,
    bullets: Query<Entity, With<player::shooting::bullet::Bullet>>,
    rockets: Query<Entity, With<player::shooting::rocket::Rocket>>,
    rocket_explosions: Query<Entity, With<player::shooting::rocket_explosion::RocketExplosion>>,
    health_bars: Query<Entity, With<player::health::HealthBar>>,
    reload_bars: Query<Entity, With<player::shooting::reloading::ReloadBar>>,
    obstacles: Query<Entity, With<map::obstacle::Obstacle>>,
) {
    for player in &players {
        commands.entity(player).despawn_recursive();
    }

    for bullet in &bullets {
        commands.entity(bullet).despawn_recursive();
    }

    for rocket in &rockets {
        commands.entity(rocket).despawn_recursive();
    }

    for rocket_explosion in &rocket_explosions {
        commands.entity(rocket_explosion).despawn_recursive();
    }

    for health_bar in &health_bars {
        commands.entity(health_bar).despawn_recursive();
    }

    for reload_bar in &reload_bars {
        commands.entity(reload_bar).despawn_recursive();
    }

    for obstacle in &obstacles {
        commands.entity(obstacle).despawn_recursive();
    }
}

fn purge_entities(
    mut commands: Commands,
    entities: Query<Entity, (Without<Window>, Without<BgmStage>)>,
) {
    info!("initiate the purge");

    for entity in &entities {
        // We use despawn instead of despawn_recursive because that would
        // result in the children being despawned but still in the query
        commands.entity(entity).despawn();
    }
}

fn reset_resources(
    mut round_stats: ResMut<RoundStats>,
    mut seeds: ResMut<Seeds>,
    mut score: ResMut<Score>,
    mut round_end_timer: ResMut<RoundEndTimer>,
    mut round_start_timer: ResMut<RoundStartTimer>,
    mut hide_screen_timer: ResMut<HideScreenTimer>,
    mut playback_states: ResMut<PlaybackStates>,
    mut ready: ResMut<Ready>,
) {
    *round_stats = RoundStats::default();
    *seeds = Seeds::default();
    *score = Score::default();
    *round_end_timer = RoundEndTimer::default();
    *round_start_timer = RoundStartTimer::default();
    *hide_screen_timer = HideScreenTimer::default();
    *playback_states = PlaybackStates::default();
    *ready = Ready::default();
}

fn purge_network_resources(world: &mut World) {
    if world.contains_resource::<AceSocket>() {
        world.remove_resource::<AceSocket>();
    }
    if world.contains_resource::<Session<GgrsConfig>>() {
        world.remove_resource::<Session<GgrsConfig>>();
    }
    if world.contains_resource::<LocalPlayerHandle>() {
        world.remove_resource::<LocalPlayerHandle>();
    }
}

pub struct WorldClearPlugin;

impl Plugin for WorldClearPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::MainMenu),
            (purge_entities, reset_resources, purge_network_resources).chain(),
        )
        .add_systems(OnEnter(RollbackState::RoundStart), clear_world);
    }
}
