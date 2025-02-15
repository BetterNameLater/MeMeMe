use crate::items::primitive::interactible::Interactible;
use crate::items::primitive::is_activated::IsActivated;
use crate::items::primitive::item::Item;
use crate::items::primitive::pressable::Pressable;
use bevy::prelude::Component;

/// Represent a button item
/// The button item is clicked, making it [`crate::items::components::is_activated::IsActivated`]
/// It waits a (Variable ??TODO?) cool-down time and unset [`crate::items::components::is_activated::IsActivated`]
/// During the cool-down time no-body can interact with the button
#[derive(Component)]
#[require(Item, Pressable, IsActivated, Interactible)]
pub struct Button;
