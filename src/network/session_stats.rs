use bevy::prelude::*;
use bevy_ggrs::{ggrs::NetworkStats, *};

use super::{ggrs_config::PLAYER_COUNT, GgrsConfig};

#[derive(Resource)]
pub struct SessionStats {
    pub network_stats: Vec<NetworkStats>,
}

impl Default for SessionStats {
    fn default() -> Self {
        Self {
            network_stats: vec![NetworkStats::default(); PLAYER_COUNT],
        }
    }
}

pub fn update_session_stats(
    mut session: ResMut<Session<GgrsConfig>>,
    mut session_stats: ResMut<SessionStats>,
) {
    match session.as_mut() {
        Session::P2P(s) => {
            for i in 0..PLAYER_COUNT {
                match s.network_stats(i) {
                    Ok(stats) => session_stats.network_stats[i] = stats,
                    Err(_) => session_stats.network_stats[i] = NetworkStats::default(),
                }
            }
        }
        _ => panic!("Expecting a P2P Session."),
    }
}
