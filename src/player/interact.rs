use crate::math::vec2i::Vec2i;
use bevy::prelude::*;

pub trait InteractEvent: Event {
    fn get_pos(&self) -> Vec2i;
    fn get_entity(&self) -> Entity;
    fn new(pos: Vec2i, entity: Entity) -> Self;
}

#[derive(Event)]
pub struct PlayerInteractEvent(InteractEventData);

impl InteractEvent for PlayerInteractEvent {
    fn get_pos(&self) -> Vec2i {
        self.0.pos
    }
    fn get_entity(&self) -> Entity {
        self.0.entity
    }
    fn new(pos: Vec2i, entity: Entity) -> Self {
        Self(InteractEventData { pos, entity })
    }
}

#[derive(Event)]
pub struct GhostInteractEvent(InteractEventData);

impl InteractEvent for GhostInteractEvent {
    fn get_pos(&self) -> Vec2i {
        self.0.pos
    }
    fn get_entity(&self) -> Entity {
        self.0.entity
    }
    fn new(pos: Vec2i, entity: Entity) -> Self {
        Self(InteractEventData { pos, entity })
    }
}

struct InteractEventData {
    pub pos: Vec2i,
    pub entity: Entity,
}
