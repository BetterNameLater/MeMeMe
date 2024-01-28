use crate::items::components::is_activated::IsActivated;
use crate::items::components::toggle::{Interact, Toggle};
use bevy::prelude::Bundle;

/// Represent a Lever (or switch) item
/// When a person `Interact` it activate itself
#[derive(Bundle, Default)]
pub struct LeverBundle {
    pub toggle: Toggle<Interact>,
    pub is_activated: IsActivated,
}
