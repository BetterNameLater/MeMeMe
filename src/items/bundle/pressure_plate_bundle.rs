use crate::items::components::enterable::EnterAble;
use crate::items::components::people_on::PeopleOn;
use bevy::prelude::Bundle;

#[derive(Bundle)]
pub struct PressurePlateBundle {
    pub enterable: EnterAble,
    pub people_on: PeopleOn,
}
