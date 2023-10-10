use bevy::prelude::*;
use bevy_matchbox::prelude::{PeerId, PeerState};

use super::socket::AceSocket;
use crate::game_logic::Seeds;
use crate::GameState;

#[derive(Debug, Clone, Event)]
pub struct PeerConnectionEvent {
    pub id: PeerId,
    pub state: PeerState,
}

#[derive(Debug, Clone, Event)]
pub struct SeedBroadcast(pub u32);

pub fn handle_seed_broadcast(
    mut socket: ResMut<AceSocket>,
    mut peer_events: EventReader<PeerConnectionEvent>,
    seed: Res<Seeds>,
) {
    if peer_events.iter().any(|event| {
        matches!(
            event,
            PeerConnectionEvent {
                state: PeerState::Connected,
                ..
            }
        )
    }) {
        socket.broadcast_tcp_seed(SeedBroadcast(seed.0[0].seed));
    }
}

pub(crate) struct OnlinePeerPlugin;
impl Plugin for OnlinePeerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PeerConnectionEvent>()
            .add_event::<SeedBroadcast>()
            .add_systems(
                Update,
                handle_seed_broadcast.run_if(in_state(GameState::Matchmaking)),
            );
    }
}
