use crate::items::components::enterable::EnterAble;
use crate::items::components::toggle::{Enter, Toggle};
use bevy::prelude::Bundle;

#[derive(Bundle)]
pub struct PressurePlateOnOffBundle {
    pub enterable: EnterAble,
    pub toggle_enter: Toggle<Enter>,
}
