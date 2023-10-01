use bevy::prelude::*;

const BORDER_MIN_X: f32 = -800.0;
const BORDER_MAX_X: f32 = 800.0;
const BORDER_MIN_Y: f32 = -448.0;
const BORDER_MAX_Y: f32 = 448.0;

pub fn spawn_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("stone-background.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(1600.0, 896.0), 1, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn(SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, -1000.0)),
        ..default()
    });
}

pub fn outside_of_borders(target_position: Vec3) -> bool {
    if target_position.x < BORDER_MIN_X || target_position.x > BORDER_MAX_X {
        return true;
    } else if target_position.y < BORDER_MIN_Y || target_position.y > BORDER_MAX_Y {
        return true;
    }
    false
}
