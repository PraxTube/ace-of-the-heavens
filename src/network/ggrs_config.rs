use bevy_ggrs::ggrs::{Config, DesyncDetection, SessionBuilder};
use bevy_matchbox::prelude::PeerId;

pub const GGRS_FPS: usize = 60;
pub const PLAYER_COUNT: usize = 2;
pub const INPUT_DELAY: usize = 2;
pub const MAX_PREDICTION_FRAMES: usize = 8;
pub const MAX_FRAME_BEHIND: usize = 10;

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
