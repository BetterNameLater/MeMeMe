use crate::items::components::people_on::PeopleOn;
use bevy::prelude::Bundle;

#[derive(Bundle)]
pub struct PressurePlateBundle {
    pub people_on: PeopleOn,
}
