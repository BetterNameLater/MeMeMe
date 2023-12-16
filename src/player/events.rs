use bevy::prelude::Event;
use crate::math::vec2i::Vec2i;

#[derive(Event)]
pub struct RewindEvent;

#[derive(Event)]
pub struct OnEnterEvent(pub Vec2i);
