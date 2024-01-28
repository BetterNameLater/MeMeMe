use crate::items::components::enterable::EnterAble;
use crate::items::components::toggle::{Enter, Toggle};
use bevy::prelude::Bundle;

/// Represent a Pressure Plate On Off (light) item
/// When more a person enter on the plate toggle [`crate::items::components::is_activated::IsActivated`]
#[derive(Bundle, Default)]
pub struct PressurePlateOnOffBundle {
    pub enterable: EnterAble,
    pub toggle_enter: Toggle<Enter>,
}
