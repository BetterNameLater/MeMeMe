use bevy::prelude::*;

/// Tag component used to flag the level entity as the level.
/// We spawn all entities in a level in this.
/// It requires a sprite, to allow his children to render correctly
#[derive(Component)]
#[require(Sprite)]
pub struct LevelTag;
