use crate::items::components::enterable::EnterAble;
use crate::items::components::teleporter::Teleporter;
use bevy::prelude::Bundle;

/// Represent a Teleporter item
/// When a person enter the teleporter, it moves the person to the destination
#[derive(Bundle)]
pub struct TeleporterBundle {
    pub enterable: EnterAble,
    pub teleporter: Teleporter,
}
