use bevy::prelude::*;
use maths::Vec2i;

pub struct NewPositionEventData {
    pub before: Vec2i,
    pub now: Vec2i,
    pub entity: Entity,
}
