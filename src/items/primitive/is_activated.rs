use bevy::prelude::Component;

/// Component representing the state of an item. Is it enable, can we interact with it ?
/// - Door
/// - ...
#[derive(Component, Default)]
pub struct IsActivated(pub bool);

// TODO reset on rewind !!
