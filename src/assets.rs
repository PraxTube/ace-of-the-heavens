use serde::Deserialize;

use bevy::prelude::*;
use bevy::reflect::{TypePath, TypeUuid};
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    // PLAYER
    #[asset(path = "player/plane1.png")]
    pub player_1: Handle<Image>,
    #[asset(path = "player/plane2.png")]
    pub player_2: Handle<Image>,
    #[asset(path = "player/plane_white.png")]
    pub plane_white: Handle<Image>,

    // PROJECTILE
    #[asset(path = "projectiles/bullet.png")]
    pub bullet: Handle<Image>,
    #[asset(path = "projectiles/rocket1.png")]
    pub rocket1: Handle<Image>,
    #[asset(path = "projectiles/rocket2.png")]
    pub rocket2: Handle<Image>,

    // GFX
    #[asset(texture_atlas(tile_size_x = 32.0, tile_size_y = 32.0, columns = 8, rows = 1))]
    #[asset(path = "gfx/explosion.png")]
    pub explosion: Handle<TextureAtlas>,

    // MAP
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

    // UI
    #[asset(path = "ui/white-pixel.png")]
    pub white_pixel: Handle<Image>,
    #[asset(path = "ui/score-full.png")]
    pub score_full: Handle<Image>,
    #[asset(path = "ui/score-empty.png")]
    pub score_empty: Handle<Image>,

    // SOUND
    #[asset(path = "sounds/bullet_shot.ogg")]
    pub bullet_shot: Handle<AudioSource>,
    #[asset(path = "sounds/overheat.ogg")]
    pub overheat: Handle<AudioSource>,
    #[asset(path = "sounds/reload.ogg")]
    pub reload: Handle<AudioSource>,

    #[asset(path = "sounds/rocket_spawn_sound.ogg")]
    pub rocket_spawn_sound: Handle<AudioSource>,
    #[asset(path = "sounds/rocket_shot.ogg")]
    pub rocket_shot: Handle<AudioSource>,
    #[asset(path = "sounds/rocket_reload.ogg")]
    pub rocket_reload: Handle<AudioSource>,
    #[asset(path = "sounds/explosion.ogg")]
    pub explosion_sound: Handle<AudioSource>,

    #[asset(path = "sounds/damage.ogg")]
    pub damage_sound: Handle<AudioSource>,
    #[asset(path = "sounds/death.ogg")]
    pub death_sound: Handle<AudioSource>,
    #[asset(path = "sounds/dodge.ogg")]
    pub dodge_sound: Handle<AudioSource>,
    #[asset(path = "sounds/dodge_refresh.ogg")]
    pub dodge_refresh: Handle<AudioSource>,

    #[asset(path = "sounds/round-start-sound.ogg")]
    pub round_start_sound: Handle<AudioSource>,

    #[asset(path = "music/bgm.ogg")]
    pub bgm: Handle<AudioSource>,
    #[asset(path = "music/bgm-match-point.ogg")]
    pub bgm_match_point: Handle<AudioSource>,

    // FONT
    #[asset(path = "fonts/PressStart2P.ttf")]
    pub font: Handle<Font>,

    // MISC
    #[asset(path = "turn-credentials.toml")]
    pub turn_credentials: Handle<TurnCredentials>,
}

#[derive(Clone, Deserialize, TypeUuid, TypePath, Default)]
#[uuid = "413be529-bfeb-41b3-9db0-4b8b380a2c46"]
pub struct TurnCredentials {
    pub username: String,
    pub credential: String,
}
