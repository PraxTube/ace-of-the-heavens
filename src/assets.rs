use serde::Deserialize;

use bevy::prelude::*;
use bevy::reflect::{TypePath, TypeUuid};
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
    #[asset(texture_atlas(tile_size_x = 32.0, tile_size_y = 32.0, columns = 8, rows = 1))]
    #[asset(path = "explosion.png")]
    pub explosion: Handle<TextureAtlas>,

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

    #[asset(path = "sounds/bullet_shot.ogg")]
    pub bullet_shot: Handle<AudioSource>,
    #[asset(path = "sounds/overheat.ogg")]
    pub overheat: Handle<AudioSource>,
    #[asset(path = "sounds/reload.ogg")]
    pub reload: Handle<AudioSource>,

    #[asset(path = "sounds/rocket_shot.ogg")]
    pub rocket_shot: Handle<AudioSource>,
    #[asset(path = "sounds/explosion.ogg")]
    pub explosion_sound: Handle<AudioSource>,

    #[asset(path = "sounds/damage.ogg")]
    pub damage_sound: Handle<AudioSource>,
    #[asset(path = "sounds/death.ogg")]
    pub death_sound: Handle<AudioSource>,

    #[asset(path = "sounds/dodge.ogg")]
    pub dodge_sound: Handle<AudioSource>,

    #[asset(path = "sounds/round-start-sound.ogg")]
    pub round_start_sound: Handle<AudioSource>,

    #[asset(path = "turn-credentials.toml")]
    pub turn_credentials: Handle<TurnCredentials>,
}

#[derive(Clone, Deserialize, TypeUuid, TypePath, Default)]
#[uuid = "413be529-bfeb-41b3-9db0-4b8b380a2c46"]
pub struct TurnCredentials {
    pub username: String,
    pub credential: String,
}
