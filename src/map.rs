use bevy::prelude::*;
use maths::Vec2i;
use std::collections::HashMap;

/// Stores items position
#[derive(Component, Default)]
pub struct ObjectMap(pub HashMap<Vec2i, Entity>);

/// Used to hold background cells and walls as [`bevy::hierarchy::Children`]
#[derive(Component)]
pub struct WorldMap;
