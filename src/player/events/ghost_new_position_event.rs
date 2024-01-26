use crate::math::vec2i::Vec2i;
use crate::player::events::new_position_event::NewPositionEvent;
use crate::player::events::new_position_event::NewPositionEventData;
use bevy::prelude::{Entity, Event};

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
