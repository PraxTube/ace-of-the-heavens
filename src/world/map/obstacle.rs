use bevy::prelude::*;

const BORDER_MIN_X: f32 = -800.0;
const BORDER_MAX_X: f32 = 800.0;
const BORDER_MIN_Y: f32 = -448.0;
const BORDER_MAX_Y: f32 = 448.0;

#[derive(Component)]
pub struct Obstacle {
    min_pos: Vec2,
    max_pos: Vec2,
    global_pos: Vec2,
}

#[derive(Component, Reflect, Default)]
pub struct CollisionEntity {
    pub disabled: bool,
}

impl Obstacle {
    pub fn new(min_pos: Vec2, max_pos: Vec2, global_pos: Vec2) -> Obstacle {
        Obstacle {
            min_pos,
            max_pos,
            global_pos,
        }
    }
}

fn collision(obstacle: &Obstacle, other_pos: Vec3) -> bool {
    let circle_pos = other_pos.truncate();
    let closest_point = circle_pos.clamp(
        obstacle.min_pos + obstacle.global_pos,
        obstacle.max_pos + obstacle.global_pos,
    );
    let distance = circle_pos.distance_squared(closest_point);
    distance < 1.0
}

fn outside_of_borders(target_position: Vec3) -> bool {
    if target_position.x < BORDER_MIN_X
        || target_position.x > BORDER_MAX_X
        || target_position.y < BORDER_MIN_Y
        || target_position.y > BORDER_MAX_Y
    {
        return true;
    }
    false
}

pub fn disable_collision_entities(
    mut collision_entities: Query<(&mut CollisionEntity, &Transform)>,
    obstacles: Query<&Obstacle>,
) {
    for (mut collision_entity, collision_transform) in &mut collision_entities {
        if outside_of_borders(collision_transform.translation) {
            collision_entity.disabled = true;
            continue;
        }

        for obstacle in &obstacles {
            if collision(obstacle, collision_transform.translation) {
                collision_entity.disabled = true;
            }
        }
    }
}
