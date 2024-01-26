use bevy::prelude::Component;

/// Component representing the state of an item. Is it enable, can we interact with it ?
/// - Door
/// - ...
#[derive(Component)]
pub struct IsActivated(pub bool);
