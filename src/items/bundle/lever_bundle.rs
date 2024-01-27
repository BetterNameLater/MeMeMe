use crate::items::components::toggle::{Interact, Toggle};
use bevy::prelude::Bundle;

/// Represent a Lever (or switch) item
#[derive(Bundle)]
pub struct LeverBundle {
    pub toggle: Toggle<Interact>,
}
