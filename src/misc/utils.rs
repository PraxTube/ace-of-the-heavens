use bevy::prelude::*;

pub fn quat_from_vec3(direction: Vec3) -> Quat {
    Quat::from_euler(
        EulerRot::XYZ,
        0.0,
        0.0,
        Vec2::X.angle_between(direction.truncate()),
    )
}
