use bevy::prelude::*;

/// Is it on ? Does he meet the conditions to be on ?
/// - Pressure Plate
/// - ...
#[derive(Component)]
pub struct IsUsable(pub bool);
