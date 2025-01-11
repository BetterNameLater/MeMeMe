use crate::items::primitive::enterable::EnterAble;
use crate::items::primitive::is_activated::IsActivated;
use crate::items::primitive::toggle::{Enter, Toggle};
use bevy::prelude::Bundle;

/// Represent a Pressure Plate On Off (light) item
/// When more a person enter on the plate toggle [`crate::items::components::is_activated::IsActivated`]
#[derive(Bundle, Default)]
pub struct PressurePlateOnOffBundle {
    pub enterable: EnterAble,
    pub toggle_enter: Toggle<Enter>,
    pub is_activated: IsActivated,
}
