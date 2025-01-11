use crate::items::primitive::enterable::EnterAble;
use crate::items::primitive::teleporter::Teleporter;
use bevy::prelude::Bundle;

/// Represent a Teleporter item
/// When a person enter the teleporter, it moves the person to the destination
#[derive(Bundle, Default)]
pub struct TeleporterBundle {
    pub enterable: EnterAble,
    pub teleporter: Teleporter,
}
