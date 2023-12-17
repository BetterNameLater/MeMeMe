use crate::math::vec2i::Vec2i;
use bevy::prelude::Event;

#[derive(Event)]
pub struct RewindEvent;

#[derive(Event)]
pub struct PlayerNewPositionEvent(pub Vec2i);

#[derive(Event)]
pub struct GhostNewPositionEvent(pub Vec2i);
