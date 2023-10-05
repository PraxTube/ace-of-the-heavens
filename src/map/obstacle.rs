use bevy::prelude::*;
use bevy_ggrs::prelude::*;

use crate::{debug::DebugTransform, GameAssets};

#[derive(Component)]
pub struct Obstacle {
    min_pos: Vec2,
    max_pos: Vec2,
    global_pos: Vec3,
    radius_square: f32,
}

impl Obstacle {
    fn new(min_pos: Vec2, max_pos: Vec2, global_pos: Vec3) -> Obstacle {
        let radius_square = (min_pos - max_pos).dot(min_pos - max_pos) / 4.0;
        Obstacle {
            min_pos,
            max_pos,
            global_pos,
            radius_square,
        }
    }
}

fn spawn_obstacle(
    commands: &mut Commands,
    texture_atlas_handle: Handle<TextureAtlas>,
    spawn_position: Vec3,
) {
    let transform = Transform::from_translation(spawn_position);
    commands
        .spawn((
            Obstacle::new(Vec2::new(-32.0, 0.0), Vec2::new(32.0, 48.0), spawn_position),
            DebugTransform::new(&transform),
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(0),
                transform,
                ..default()
            },
        ))
        .add_rollback();
}

pub fn spawn_obstacles(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    assets: Res<GameAssets>,
) {
    let texture_handle = assets.obstacle.clone();
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 96.0), 1, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let positions = [
        Vec3::new(0.0, 0.0, -10.0),
        Vec3::new(-550.0, -250.0, -10.0),
        Vec3::new(550.0, -250.0, -10.0),
        Vec3::new(550.0, 250.0, -10.0),
        Vec3::new(-550.0, 250.0, -10.0),
    ];

    for position in positions {
        spawn_obstacle(&mut commands, texture_atlas_handle.clone(), position);
    }
}

pub fn collision(obstacle: &Obstacle, other_pos: Vec3, other_radius: f32) -> bool {
    if Vec2::distance_squared(obstacle.global_pos.truncate(), other_pos.truncate())
        > obstacle.radius_square + other_radius * other_radius
    {
        return false;
    }

    let mut x_overlap = false;
    let mut y_overlap = false;
    if other_pos.x + other_radius > obstacle.global_pos.x + obstacle.min_pos.x
        && other_pos.x - other_radius < obstacle.global_pos.x + obstacle.max_pos.x
    {
        x_overlap = true;
    }
    if other_pos.y + other_radius > obstacle.global_pos.y + obstacle.min_pos.y
        && other_pos.y - other_radius < obstacle.global_pos.y + obstacle.max_pos.y
    {
        y_overlap = true;
    }
    x_overlap && y_overlap
}
