use crate::math::vec2i::Vec2i;
use bevy::prelude::*;

#[derive(Event)]
pub struct RewindEvent;

#[derive(Event)]
pub struct PlayerNewPositionEvent {
    pub before: Vec2i,
    pub now: Vec2i,
	pub player: Entity,
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
