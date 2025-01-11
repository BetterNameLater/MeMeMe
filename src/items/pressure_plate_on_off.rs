use crate::items::primitive::enterable::EnterAble;
use crate::items::primitive::is_activated::IsActivated;
use crate::items::primitive::toggle::{Enter, Toggle};
use bevy::prelude::Component;

/// Represent a Pressure Plate On Off (light) item
/// When more a person enter on the plate toggle [`crate::items::components::is_activated::IsActivated`]

#[derive(Component)]
#[require(EnterAble, Toggle<Enter>, IsActivated)]
pub struct PressurePlateOnOff;
