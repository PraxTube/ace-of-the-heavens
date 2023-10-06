use bevy::prelude::*;

use crate::GameAssets;

pub const BORDER_MIN_X: f32 = -800.0;
pub const BORDER_MAX_X: f32 = 800.0;
pub const BORDER_MIN_Y: f32 = -448.0;
pub const BORDER_MAX_Y: f32 = 448.0;

pub fn spawn_background(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    assets: Res<GameAssets>,
) {
    let texture_handle = assets.background.clone();
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(1600.0, 896.0), 1, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn(SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, -1000.0)),
        ..default()
    });
}
