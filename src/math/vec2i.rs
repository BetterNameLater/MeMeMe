use bevy::math::{Vec2, Vec3};
use bevy::prelude::{Reflect, Transform};
use serde::Deserialize;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Default, Deserialize, Debug, Reflect)]
pub struct Vec2i {
    pub x: i32,
    pub y: i32,
}

impl From<Vec3> for Vec2i {
    fn from(vec3: Vec3) -> Self {
        Self::new(vec3.x as i32, vec3.y as i32)
    }
}

impl From<Vec2> for Vec2i {
    fn from(vec3: Vec2) -> Self {
        Self::new(vec3.x as i32, vec3.y as i32)
    }
}

impl From<Vec2i> for Vec2 {
    fn from(val: Vec2i) -> Self {
        Vec2::new(val.x as f32, val.y as f32)
    }
}

impl Vec2i {
    pub fn new(x: i32, y: i32) -> Vec2i {
        Vec2i { x, y }
    }
    pub fn to_transform(self, z: f32) -> Transform {
        Transform::from_xyz(self.x as f32, self.y as f32, z)
    }
}
