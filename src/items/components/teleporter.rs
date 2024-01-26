use crate::math::vec2i::Vec2i;
use bevy::prelude::Component;

#[derive(Component)]
pub struct Teleporter(pub Vec2i);
