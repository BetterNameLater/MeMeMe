use crate::math::vec2i::Vec2i;
use bevy::prelude::*;

#[derive(Event)]
pub struct RewindEvent;

pub trait NewPositionEvent: Event {
    fn get_before(&self) -> Vec2i;
    fn get_now(&self) -> Vec2i;
    fn get_entity(&self) -> Entity;
    fn new(before: Vec2i, now: Vec2i, entity: Entity) -> Self;
}

#[derive(Event)]
pub struct PlayerNewPositionEvent(NewPositionEventData);

impl NewPositionEvent for PlayerNewPositionEvent {
    fn get_before(&self) -> Vec2i {
        self.0.before
    }
    fn get_now(&self) -> Vec2i {
        self.0.now
    }
    fn get_entity(&self) -> Entity {
        self.0.entity
    }
    fn new(before: Vec2i, now: Vec2i, entity: Entity) -> Self {
        Self(NewPositionEventData {
            before,
            now,
            entity,
        })
    }
}

#[derive(Event)]
pub struct GhostNewPositionEvent(NewPositionEventData);

impl NewPositionEvent for GhostNewPositionEvent {
    fn get_before(&self) -> Vec2i {
        self.0.before
    }
    fn get_now(&self) -> Vec2i {
        self.0.now
    }
    fn get_entity(&self) -> Entity {
        self.0.entity
    }
    fn new(before: Vec2i, now: Vec2i, entity: Entity) -> Self {
        Self(NewPositionEventData {
            before,
            now,
            entity,
        })
    }
}

struct NewPositionEventData {
    pub before: Vec2i,
    pub now: Vec2i,
    pub entity: Entity,
}
