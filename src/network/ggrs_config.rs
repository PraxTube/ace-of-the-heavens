use bevy::log::error;
use bevy_ggrs::ggrs::{Config, DesyncDetection, SessionBuilder};
use bevy_matchbox::{matchbox_socket::RtcIceServerConfig, prelude::PeerId};

use crate::assets::TurnCredentials;

pub const GGRS_FPS: usize = 60;
pub const PLAYER_COUNT: usize = 2;
pub const INPUT_DELAY: usize = 2;
pub const MAX_PREDICTION_FRAMES: usize = 38;
pub const MAX_FRAME_BEHIND: usize = 40;

#[derive(Debug)]
pub struct GgrsConfig;

impl Config for GgrsConfig {
    type Input = u8;
    type State = u8;
    type Address = PeerId;
}

impl GgrsConfig {
    pub fn new_builder() -> SessionBuilder<Self> {
        SessionBuilder::<Self>::new()
            .with_fps(GGRS_FPS)
            .expect("invalid FPS, must be above 0")
            .with_num_players(PLAYER_COUNT)
            .with_input_delay(INPUT_DELAY)
            .with_max_prediction_window(MAX_PREDICTION_FRAMES)
            .with_max_frames_behind(MAX_FRAME_BEHIND)
            .expect("couldn't set max frames behind")
            .with_desync_detection_mode(DesyncDetection::On { interval: 10 })
    }
}

pub fn get_rtc_ice_server_config(turn_credentials: Option<&TurnCredentials>) -> RtcIceServerConfig {
    let turn_credentials = match turn_credentials {
        Some(tc) => tc,
        None => {
            error!("unable to fetch turn server credentials from assets! This might lead to online multiplayer not working");
            return RtcIceServerConfig::default();
        }
    };

    if turn_credentials.username == "Your Turn Username"
        || turn_credentials.credential == "Your Turn Credential/Password"
    {
        error!(
            "using dummy values for turn credentials: username: {}, credential: {}",
            turn_credentials.username, turn_credentials.credential
        );
    }

    RtcIceServerConfig {
        urls: vec![
            "stun:stun.l.google.com:19302".to_string(),
            "stun:stun1.l.google.com:19302".to_string(),
            "stun:fr-turn3.xirsys.com".to_string(),
            "turn:fr-turn3.xirsys.com:80?transport=udp".to_string(),
            "turn:fr-turn3.xirsys.com:3478?transport=udp".to_string(),
            "turn:fr-turn3.xirsys.com:80?transport=tcp".to_string(),
            "turn:fr-turn3.xirsys.com:3478?transport=tcp".to_string(),
            "turns:fr-turn3.xirsys.com:443?transport=tcp".to_string(),
            "turns:fr-turn3.xirsys.com:5349?transport=tcp".to_string(),
        ],
        username: Some(turn_credentials.username.clone()),
        credential: Some(turn_credentials.credential.clone()),
    }
}
