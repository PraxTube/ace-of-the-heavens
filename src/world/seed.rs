use chrono::Utc;

use bevy::prelude::*;
use bevy_matchbox::prelude::PeerId;

use crate::{
    network::{ggrs_config::PLAYER_COUNT, session::start_matchbox_socket},
    GameState,
};

#[derive(Default, Debug)]
pub struct SeedHandle {
    pub handle: Option<PeerId>,
    pub seed: u32,
}

impl SeedHandle {
    fn new(handle: Option<PeerId>, seed: u32) -> SeedHandle {
        SeedHandle { handle, seed }
    }
}

#[derive(Resource, Default, Debug)]
pub struct Seeds(pub Vec<SeedHandle>);

#[derive(Resource, Default, Debug)]
pub struct Seed {
    pub seed: u64,
}

fn initiate_seed(mut seeds: ResMut<Seeds>) {
    let current_time = Utc::now().timestamp() as u32;
    seeds.0.push(SeedHandle::new(None, current_time));
}

fn setup_seed(mut seed: ResMut<Seed>, seeds: Res<Seeds>) {
    if seeds.0.len() != PLAYER_COUNT {
        panic!(
            "we didn't receive the correct amount of seeds from our peer\nReceived {} seeds",
            seeds.0.len()
        );
    }
    *seed = Seed {
        seed: determine_seed(&seeds) as u64,
    };
}

pub fn determine_seed(seeds: &Res<Seeds>) -> u32 {
    let mut smallest_seed = seeds.0[0].seed;
    for seed in &seeds.0 {
        if seed.seed < smallest_seed {
            smallest_seed = seed.seed;
        }
    }
    smallest_seed
}

pub struct WorldSeedPlugin;

impl Plugin for WorldSeedPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Matchmaking),
            initiate_seed.before(start_matchbox_socket),
        )
        .init_resource::<Seeds>()
        .init_resource::<Seed>()
        .add_systems(OnExit(GameState::Matchmaking), setup_seed);
    }
}
