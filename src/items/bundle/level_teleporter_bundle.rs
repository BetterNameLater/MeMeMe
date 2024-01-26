use crate::items::components::enterable::EnterAble;
use crate::items::components::level_teleporter::LevelTeleporter;
use bevy::prelude::Bundle;

#[derive(Bundle)]
pub struct LevelTeleporterBundle {
    pub level_teleporter: LevelTeleporter,
    pub enterable: EnterAble,
}
