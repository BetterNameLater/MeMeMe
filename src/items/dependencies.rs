use bevy::prelude::*;

#[derive(Component)]
pub struct Dependencies(pub Vec<Entity>);
