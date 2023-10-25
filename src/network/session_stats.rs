use bevy::prelude::*;
use bevy_ggrs::{ggrs::NetworkStats, *};

use super::{ggrs_config::PLAYER_COUNT, GgrsConfig};

#[derive(Resource, Default)]
pub struct SessionStats {
    /// The length of the queue containing UDP packets which have not yet been acknowledged by the end client.
    /// The length of the send queue is a rough indication of the quality of the connection.
    /// The longer the send queue, the higher the round-trip time between the
    /// clients. The send queue will also be longer than usual during high packet loss situations.
    pub send_queue_len: usize,
    /// The roundtrip packet transmission time as calculated by GGRS.
    pub ping: u128,
    /// The estimated bandwidth used between the two clients, in kilobits per second.
    pub kbps_sent: usize,

    /// The number of frames GGRS calculates that the local client is behind the remote client at this instant in time.
    /// For example, if at this instant the current game client is running frame 1002 and the remote game client is running frame 1009,
    /// this value will mostly likely roughly equal 7.
    pub local_frames_behind: i32,
    /// The same as [`local_frames_behind`], but calculated from the perspective of the remote player.
    ///
    /// [`local_frames_behind`]: #structfield.local_frames_behind
    pub remote_frames_behind: i32,
}

impl From<NetworkStats> for SessionStats {
    fn from(network_stats: NetworkStats) -> Self {
        Self {
            send_queue_len: network_stats.send_queue_len,
            ping: network_stats.ping,
            kbps_sent: network_stats.kbps_sent,
            local_frames_behind: network_stats.local_frames_behind,
            remote_frames_behind: network_stats.remote_frames_behind,
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
                    Ok(stats) => *session_stats = SessionStats::from(stats),
                    Err(_) => {}
                }
            }
        }
        _ => panic!("Expecting a P2P Session."),
    }
}
