use crate::items::primitive::is_activated::IsActivated;
use crate::items::primitive::item::Item;
use crate::items::primitive::toggle::{Interact, Toggle};
use bevy::prelude::Component;

/// Represent a Lever (or switch) item
/// When a person `Interact` it activate itself
#[derive(Component)]
#[require(Item, IsActivated, Toggle<Interact>)]
pub struct Lever;
