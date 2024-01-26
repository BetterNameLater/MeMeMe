use bevy::prelude::Bundle;
use crate::items::components::people_on::PeopleOn;

#[derive(Bundle)]
pub struct PressurePlateOnOffBundle {
    pub people_on: PeopleOn,
}
