use crate::items::components::teleporter::Teleporter;
use bevy::prelude::Bundle;

#[derive(Bundle)]
pub struct TeleporterBundle {
    pub teleporter: Teleporter,
}
