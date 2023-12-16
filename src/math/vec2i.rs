use serde::Deserialize;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Default, Deserialize)]
pub struct Vec2i {
    pub x: i32,
    pub y: i32,
}

impl Vec2i {
    pub fn from(x: i32, y: i32) -> Vec2i {
        Vec2i { x, y }
    }
}
