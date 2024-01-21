use crate::math::vec2i::Vec2i;
use bevy::prelude::*;

#[derive(Event, Debug)]
pub struct OnEnterEvent {
    pub position: Vec2i,
    pub item: Entity,
    pub person: Entity,
}
