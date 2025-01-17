use bevy::prelude::*;
use maths::Vec2i;

#[derive(Event, Debug)]
pub struct OnEnterEvent {
    pub _position: Vec2i,
    pub item: Entity,
    pub person: Entity,
}

#[derive(Event, Debug)]
pub struct OnExitEvent {
    pub _position: Vec2i,
    pub item: Entity,
    pub person: Entity,
}
