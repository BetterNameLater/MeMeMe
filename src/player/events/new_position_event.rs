use crate::math::vec2i::Vec2i;
use bevy::prelude::*;

pub struct NewPositionEventData {
    pub before: Vec2i,
    pub now: Vec2i,
    pub entity: Entity,
}
