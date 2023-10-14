use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(path = "plane1.png")]
    pub player_1: Handle<Image>,
    #[asset(path = "plane2.png")]
    pub player_2: Handle<Image>,
    #[asset(path = "bullet.png")]
    pub bullet: Handle<Image>,
    #[asset(path = "rocket1.png")]
    pub rocket1: Handle<Image>,
    #[asset(path = "rocket2.png")]
    pub rocket2: Handle<Image>,
    #[asset(path = "explosion.png")]
    pub explosion: Handle<Image>,

    #[asset(path = "map/background.png")]
    pub background: Handle<Image>,
    #[asset(path = "map/walls/wall-1-1.png")]
    pub wall_1_1: Handle<Image>,
    #[asset(path = "map/walls/wall-2-2.png")]
    pub wall_2_2: Handle<Image>,
    #[asset(path = "map/walls/wall-1-5.png")]
    pub wall_1_5: Handle<Image>,
    #[asset(path = "map/walls/wall-5-1.png")]
    pub wall_5_1: Handle<Image>,
    #[asset(path = "map/walls/wall-1-10.png")]
    pub wall_1_10: Handle<Image>,

    #[asset(path = "ui/white-pixel.png")]
    pub white_pixel: Handle<Image>,
    #[asset(path = "ui/score-full.png")]
    pub score_full: Handle<Image>,
    #[asset(path = "ui/score-empty.png")]
    pub score_empty: Handle<Image>,

    #[asset(path = "fonts/PressStart2P.ttf")]
    pub font: Handle<Font>,

    #[asset(path = "sounds/explosion.ogg")]
    pub explosion_sound: Handle<AudioSource>,
}
