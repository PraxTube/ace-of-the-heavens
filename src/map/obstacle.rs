use bevy::prelude::*;

#[derive(Component)]
pub struct Obstacle {
    min_pos: Vec2,
    max_pos: Vec2,
    global_pos: Vec3,
}

impl Obstacle {
    pub fn new(min_pos: Vec2, max_pos: Vec2, global_pos: Vec3) -> Obstacle {
        Obstacle {
            min_pos,
            max_pos,
            global_pos,
        }
    }
}

pub fn collision(obstacle: &Obstacle, other_pos: Vec3) -> bool {
    let circle_pos = other_pos.truncate();
    let closest_point = circle_pos.clamp(
        obstacle.min_pos + obstacle.global_pos.truncate(),
        obstacle.max_pos + obstacle.global_pos.truncate(),
    );
    let distance = circle_pos.distance_squared(closest_point);
    distance < 1.0
}
