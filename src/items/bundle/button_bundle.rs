use crate::items::components::toggle::{Interact, Toggle};
use bevy::prelude::Bundle;

/// Represent a button item
/// The button item is clicked, making it [`crate::items::components::is_activated::IsActivated`]
/// It waits a (Variable ??TODO?) cool-down time and unset [`crate::items::components::is_activated::IsActivated`]
/// During the cool-down time no-body can interact with the button
#[derive(Bundle)]
pub struct ButtonBundle {
    pub toggle_interact: Toggle<Interact>,
}
