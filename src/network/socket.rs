use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

use bevy::prelude::*;
use bevy::tasks::IoTaskPool;
use bevy_ggrs::ggrs;
use bevy_ggrs::ggrs::PlayerType;
use bevy_matchbox::matchbox_socket::{MessageLoopFuture, WebRtcSocket};
use bevy_matchbox::prelude::{MultipleChannels, PeerId};

#[derive(Resource, Debug, Clone)]
pub struct AceSocket(pub Arc<RwLock<WebRtcSocket<MultipleChannels>>>);

fn try_into_array(boxed_slice: Box<[u8]>) -> [u8; 4] {
    if boxed_slice.len() != 4 {
        panic!("the given box slice must contain exactly 4 bytes");
    }
    let mut array = [0; 4];
    array.copy_from_slice(&boxed_slice);
    array
}

impl ggrs::NonBlockingSocket<PeerId> for AceSocket {
    fn send_to(&mut self, msg: &ggrs::Message, addr: &PeerId) {
        self.0
            .write()
            // if the lock is poisoned, we're already doomed, time to panic
            .expect("Failed to lock socket for sending!")
            .channel(Self::GGRS_CHANNEL)
            .send_to(msg, addr);
    }

    fn receive_all_messages(&mut self) -> Vec<(PeerId, ggrs::Message)> {
        self.0
            .write()
            // if the lock is poisoned, we're already doomed, time to panic
            .expect("Failed to lock socket for receiving!")
            .channel(Self::GGRS_CHANNEL)
            .receive_all_messages()
    }
}

impl From<(WebRtcSocket<MultipleChannels>, MessageLoopFuture)> for AceSocket {
    fn from(
        (socket, message_loop_fut): (WebRtcSocket<MultipleChannels>, MessageLoopFuture),
    ) -> Self {
        let task_pool = IoTaskPool::get();
        task_pool.spawn(message_loop_fut).detach();
        AceSocket(Arc::new(RwLock::new(socket)))
    }
}

impl AceSocket {
    pub const GGRS_CHANNEL: usize = 0;
    pub const RELIABLE_CHANNEL: usize = 1;

    pub fn send_tcp_seed(&mut self, peer: PeerId, seed: u32) {
        let bytes = Box::new(seed.to_be_bytes());
        self.inner_mut()
            .channel(Self::RELIABLE_CHANNEL)
            .send(bytes, peer);
    }

    pub fn receive_tcp_seed(&mut self) -> Vec<(PeerId, u32)> {
        self.inner_mut()
            .channel(Self::RELIABLE_CHANNEL)
            .receive()
            .into_iter()
            .map(|(id, packet)| {
                let seed = u32::from_be_bytes(try_into_array(packet));
                (id, seed)
            })
            .collect()
    }

    pub fn players(&self) -> Vec<PlayerType<PeerId>> {
        let Some(our_id) = self.inner().id() else {
            // we're still waiting for the server to initialize our id
            // no peers should be added at this point anyway
            return vec![PlayerType::Local];
        };

        // player order needs to be consistent order across all peers
        let mut ids: Vec<_> = self
            .inner()
            .connected_peers()
            .chain(std::iter::once(our_id))
            .collect();
        ids.sort();

        ids.into_iter()
            .map(|id| {
                if id == our_id {
                    PlayerType::Local
                } else {
                    PlayerType::Remote(id)
                }
            })
            .collect()
    }

    #[allow(unused)]
    pub fn inner(&self) -> RwLockReadGuard<'_, WebRtcSocket<MultipleChannels>> {
        // we don't care about handling lock poisoning
        self.0.read().expect("Failed to lock socket for reading!")
    }

    #[allow(unused)]
    pub fn inner_mut(&mut self) -> RwLockWriteGuard<'_, WebRtcSocket<MultipleChannels>> {
        // we don't care about handling lock poisoning
        self.0.write().expect("Failed to lock socket for writing!")
    }
}
