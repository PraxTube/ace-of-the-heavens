use bevy::prelude::*;
use std::hash::{Hash, Hasher};

#[derive(Reflect, Component, Default)]
#[reflect(Hash)]
pub struct DebugVec3(Vec3);
#[derive(Reflect, Component, Default)]
#[reflect(Hash)]
pub struct DebugQuat(Quat);

impl Hash for DebugVec3 {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.x.to_bits().hash(state);
        self.0.y.to_bits().hash(state);
        self.0.z.to_bits().hash(state);
    }
}

impl Hash for DebugQuat {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.x.to_bits().hash(state);
        self.0.y.to_bits().hash(state);
        self.0.z.to_bits().hash(state);
        self.0.w.to_bits().hash(state);
    }
}

#[derive(Reflect, Component, Hash, Default)]
#[reflect(Hash)]
pub struct DebugTransform {
    pub translation: DebugVec3,
    pub quat: DebugQuat,
    pub scale: DebugVec3,
}

impl DebugTransform {
    pub fn update(&mut self, t: &Transform) {
        self.translation = DebugVec3(t.translation);
        self.quat = DebugQuat(t.rotation);
        self.scale = DebugVec3(t.scale);
    }

    pub fn new(t: &Transform) -> DebugTransform {
        DebugTransform {
            translation: DebugVec3(t.translation),
            quat: DebugQuat(t.rotation),
            scale: DebugVec3(t.scale),
        }
    }
}
