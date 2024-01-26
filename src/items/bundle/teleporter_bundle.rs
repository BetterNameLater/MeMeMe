use crate::items::components::enterable::EnterAble;
use crate::items::components::teleporter::Teleporter;
use bevy::prelude::Bundle;

#[derive(Bundle)]
pub struct TeleporterBundle {
    pub enterable: EnterAble,
    pub teleporter: Teleporter,
}
