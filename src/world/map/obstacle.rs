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

fn vec_vec_collision(v1: Vec2, v2: Vec2, w1: Vec2, w2: Vec2) -> bool {
    // From https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection#Given_two_points_on_each_line
    let denominator = (v1.x - v2.x) * (w1.y - w2.y) - (v1.y - v2.y) * (w1.x - w2.x);
    // Should a tolerance range be used instead?
    if denominator.abs() == 0.0 {
        return false;
    }

    let t_numerator = (v1.x - w1.x) * (w1.y - w2.y) - (v1.y - w1.y) * (w1.x - w2.x);
    let u_numerator = (v1.x - w1.x) * (v1.y - v2.y) - (v1.y - w1.y) * (v1.x - v2.x);
    let t = t_numerator / denominator;
    let u = u_numerator / denominator;

    (0.0..=1.0).contains(&t) && (0.0..=1.0).contains(&u)
}

fn vec_obstacle_collision(v1: Vec2, v2: Vec2, o: &Obstacle) -> bool {
    let bl = o.min_pos + o.global_pos;
    let br = Vec2::new(o.max_pos.x, o.min_pos.y) + o.global_pos;
    let tl = Vec2::new(o.min_pos.x, o.max_pos.y) + o.global_pos;
    let tr = o.max_pos + o.global_pos;

    let rect_vecs = [(bl, br), (br, tr), (tr, tl), (tl, bl)];
    for (w1, w2) in rect_vecs {
        if vec_vec_collision(v1, v2, w1, w2) {
            return true;
        }
    }
    false
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

pub fn ray_obstacle_collision(
    start_pos: Vec2,
    end_pos: Vec2,
    obstacles: &Query<&Obstacle>,
) -> bool {
    for obstacle in obstacles {
        if vec_obstacle_collision(start_pos, end_pos, obstacle) {
            return true;
        }
    }
    false
}
