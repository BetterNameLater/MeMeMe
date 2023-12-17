use crate::math::vec2i::Vec2i;
use bevy::prelude::Event;

#[derive(Event)]
pub struct RewindEvent;

#[derive(Event)]
pub struct PlayerNewPositionEvent {
    pub before: Vec2i,
    pub now: Vec2i,
}

#[derive(Event)]
pub struct GhostNewPositionEvent {
    pub before: Vec2i,
    pub now: Vec2i,
}

// TODO refactor ?
pub struct NewPositionEvent {
    pub before: Vec2i,
    pub now: Vec2i,
}
