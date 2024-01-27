use crate::items::components::enterable::EnterAble;
use crate::items::components::level_teleporter::LevelTeleporter;
use bevy::prelude::Bundle;

/// Represent a LevelTeleporter item
/// The LevelTeleporter call a state change to an other level when the player enter it
#[derive(Bundle, Default)]
pub struct LevelTeleporterBundle {
    pub level_teleporter: LevelTeleporter,
    pub enterable: EnterAble,
}
