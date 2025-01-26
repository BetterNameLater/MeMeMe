use bevy::prelude::*;

/// Tag component used to flag the level entity as the level.
/// We spawn all entities in a level in this.
#[derive(Component)]
#[require(Transform)]
pub struct LevelTag;
