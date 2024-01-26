use crate::items::components::toggle::{Interact, Toggle};
use bevy::prelude::Bundle;

#[derive(Bundle)]
pub struct LeverBundle {
    pub toggle: Toggle<Interact>,
}
