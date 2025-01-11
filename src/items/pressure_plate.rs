use crate::items::primitive::enterable::EnterAble;
use crate::items::primitive::is_activated::IsActivated;
use crate::items::primitive::people_on::PeopleOn;
use bevy::prelude::Bundle;

/// Represent a Pressure Plate item
/// When more than one people is on the plate, it set [`crate::items::components::is_activated::IsActivated`]
#[derive(Bundle, Default)]
pub struct PressurePlateBundle {
    pub enterable: EnterAble,
    pub people_on: PeopleOn,
    pub is_activated: IsActivated,
}
