use crate::math::vec2i::Vec2i;
use bevy::prelude::*;

pub trait NewPositionEvent: Event {
    fn get_before(&self) -> Vec2i;
    fn get_now(&self) -> Vec2i;
    fn get_entity(&self) -> Entity;
    fn new(before: Vec2i, now: Vec2i, entity: Entity) -> Self;
}

pub struct NewPositionEventData {
    pub before: Vec2i,
    pub now: Vec2i,
    pub entity: Entity,
}
