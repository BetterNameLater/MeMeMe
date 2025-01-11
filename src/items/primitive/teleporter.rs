use crate::math::vec2i::Vec2i;
use bevy::prelude::Component;

/// Le téléporter et l'endroit où il téléporte.
#[derive(Component, Default)]
pub struct Teleporter(pub Vec2i);
