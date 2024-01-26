use crate::items::systems::toggle::{Interact, Toggle};
use bevy::prelude::Bundle;

#[derive(Bundle)]
pub struct LeverBundle {
    pub toggle: Toggle<Interact>,
}
