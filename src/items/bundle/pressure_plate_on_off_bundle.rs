use crate::items::components::enterable::EnterAble;
use bevy::prelude::Bundle;

#[derive(Bundle)]
pub struct PressurePlateOnOffBundle {
    pub enterable: EnterAble,
}
