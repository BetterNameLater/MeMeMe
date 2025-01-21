use std::ops::Mul;

use bevy::math::{Vec2, Vec3};
use bevy::prelude::{Reflect, Transform};
use schemars::JsonSchema;
use serde::Deserialize;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Default, Deserialize, Debug, Reflect, JsonSchema)]
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

impl From<(i64, i64)> for Vec2i {
    fn from((x, y): (i64, i64)) -> Self {
        Self {
            x: x as i32,
            y: y as i32,
        }
    }
}

impl Mul<i32> for Vec2i {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
