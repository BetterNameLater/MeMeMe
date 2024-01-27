use crate::items::components::toggle::{Interact, Toggle};
use bevy::prelude::Bundle;

/// Represent a Lever (or switch) item
/// When a person `Interact` it activate itself
#[derive(Bundle, Default)]
pub struct LeverBundle {
    pub toggle: Toggle<Interact>,
}
