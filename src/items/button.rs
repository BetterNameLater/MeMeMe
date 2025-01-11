use crate::items::primitive::is_activated::IsActivated;
use crate::items::primitive::pressable::Pressable;
use bevy::prelude::Bundle;

/// Represent a button item
/// The button item is clicked, making it [`crate::items::components::is_activated::IsActivated`]
/// It waits a (Variable ??TODO?) cool-down time and unset [`crate::items::components::is_activated::IsActivated`]
/// During the cool-down time no-body can interact with the button
#[derive(Bundle, Default)]
pub struct ButtonBundle {
    pub pressed: Pressable,
    pub is_activated: IsActivated,
}
