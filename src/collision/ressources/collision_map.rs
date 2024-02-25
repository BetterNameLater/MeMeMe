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
    pub fn collide(&self, position: &Vec3) -> bool {
        if let Some(_) = self.map.get(&Vec2i::from(*position)) {
            true
        } else {
            false
        }
    }

    pub fn add_collision(&mut self, position: &Vec3) {
        self.map
            .insert(Vec2i {
                    x: position.x as i32,
                    y: position.y as i32},
                CollisionMask { mask: true as u8 });
    }
}
