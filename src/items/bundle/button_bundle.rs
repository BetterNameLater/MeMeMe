use crate::items::components::toggle::{Interact, Toggle};
use bevy::prelude::Bundle;

#[derive(Bundle)]
pub struct ButtonBundle {
    pub toggle_interact: Toggle<Interact>,
}
