use bevy_ggrs::ggrs::Config;
use bevy_matchbox::prelude::PeerId;

#[derive(Debug)]
pub struct GgrsConfig;

impl Config for GgrsConfig {
    type Input = u8;
    type State = u8;
    type Address = PeerId;
}
