use bevy::{prelude::*, sprite::collide_aabb::Collision};
use bevy_ggrs::prelude::*;

use super::map::{BORDER_MAX_X, BORDER_MAX_Y, BORDER_MIN_X, BORDER_MIN_Y};
use crate::{debug::DebugTransform, GameAssets};

#[derive(Component)]
pub struct Obstacle {
    min_pos: Vec2,
    max_pos: Vec2,
    global_pos: Vec3,
    radius_square: f32,
    pub damage: u32,
}

impl Obstacle {
    fn new(min_pos: Vec2, max_pos: Vec2, global_pos: Vec3) -> Obstacle {
        let radius_square = (min_pos - max_pos).dot(min_pos - max_pos) / 4.0;
        Obstacle {
            min_pos,
            max_pos,
            global_pos,
            radius_square,
            damage: 5,
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

pub fn spawn_borders(mut commands: Commands) {
    // Top border
    commands.spawn(Obstacle::new(
        Vec2::new(BORDER_MIN_X, 0.0),
        Vec2::new(BORDER_MAX_X, 1000.0),
        Vec3::new(0.0, BORDER_MAX_Y, 0.0),
    ));
    // Bottom border
    commands.spawn(Obstacle::new(
        Vec2::new(BORDER_MIN_X, -1000.0),
        Vec2::new(BORDER_MAX_X, 0.0),
        Vec3::new(0.0, BORDER_MIN_Y, 0.0),
    ));
    // Left border
    commands.spawn(Obstacle::new(
        Vec2::new(-1000.0, BORDER_MIN_Y),
        Vec2::new(0.0, BORDER_MAX_Y),
        Vec3::new(BORDER_MIN_X, 0.0, 0.0),
    ));
    // Right border
    commands.spawn(Obstacle::new(
        Vec2::new(0.0, BORDER_MIN_Y),
        Vec2::new(1000.0, BORDER_MAX_Y),
        Vec3::new(BORDER_MAX_X, 0.0, 0.0),
    ));
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

pub fn collision(obstacle: &Obstacle, other_pos: Vec3, other_radius: f32) -> Option<Collision> {
    if Vec2::distance_squared(obstacle.global_pos.truncate(), other_pos.truncate())
        > obstacle.radius_square + other_radius * other_radius
    {
        return None;
    }

    // a is the other
    let a_min = other_pos.truncate() - Vec2::ONE * other_radius;
    let a_max = other_pos.truncate() + Vec2::ONE * other_radius;
    // b is the obstacle
    let b_min = obstacle.global_pos.truncate() + obstacle.min_pos;
    let b_max = obstacle.global_pos.truncate() + obstacle.max_pos;

    // Check if we have no collisions at all
    if !(a_min.x < b_max.x && a_max.x > b_min.x && a_min.y < b_max.y && a_max.y > b_min.y) {
        return None;
    }

    // check to see if we hit on the left or right side
    let (x_collision, x_depth) = if a_min.x < b_min.x && a_max.x > b_min.x && a_max.x < b_max.x {
        (Collision::Left, b_min.x - a_max.x)
    } else if a_min.x > b_min.x && a_min.x < b_max.x && a_max.x > b_max.x {
        (Collision::Right, a_min.x - b_max.x)
    } else {
        (Collision::Inside, -f32::INFINITY)
    };

    // check to see if we hit on the top or bottom side
    let (y_collision, y_depth) = if a_min.y < b_min.y && a_max.y > b_min.y && a_max.y < b_max.y {
        (Collision::Bottom, b_min.y - a_max.y)
    } else if a_min.y > b_min.y && a_min.y < b_max.y && a_max.y > b_max.y {
        (Collision::Top, a_min.y - b_max.y)
    } else {
        (Collision::Inside, -f32::INFINITY)
    };

    // if we had an "x" and a "y" collision, pick the "primary" side using penetration depth
    if y_depth.abs() < x_depth.abs() {
        Some(y_collision)
    } else {
        Some(x_collision)
    }
}
