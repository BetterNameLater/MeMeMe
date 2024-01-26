use bevy::prelude::Component;

/// Component with the debug name of an item
#[derive(Component, Debug)]
pub struct DebugName(pub String);
