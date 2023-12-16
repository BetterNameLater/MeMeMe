use bevy::math::Vec3;
use serde::Deserialize;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Default, Deserialize, Debug)]
pub struct Vec2i {
    pub x: i32,
    pub y: i32,
}

impl Vec2i {
    pub fn from(x: i32, y: i32) -> Vec2i {
        Vec2i { x, y }
    }

    pub fn from_vec3(vec: Vec3) -> Vec2i {
        Vec2i {
            x: vec.x as i32,
            y: vec.y as i32,
        }
    }
}
