use crate::math::vec2i::Vec2i;
use bevy::{prelude::*, utils::HashMap};

#[derive(Eq, PartialEq, Clone, Debug)]
struct CollisionMask {
    mask: u8,
}

#[derive(Resource, Debug, Default)]
pub struct CollisionMap {
    map: HashMap<Vec2i, CollisionMask>,
}

impl CollisionMap {
    pub fn collide(&self, position: &Vec2i) -> bool {
        if let Some(_) = self.map.get(position) {
            true
        } else {
            false
        }
    }
}
