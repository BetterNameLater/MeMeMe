use crate::math::vec2i::Vec2i;
use bevy::prelude::Event;

#[derive(Event)]
pub struct RewindEvent;

#[derive(Event)]
pub struct OnEnterEvent(pub Vec2i);
