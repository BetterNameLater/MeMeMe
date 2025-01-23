use bevy::prelude::*;

#[derive(Event, Debug)]
pub struct OnEnterEvent {
    pub item: Entity,
    pub person: Entity,
}

#[derive(Event, Debug)]
pub struct OnExitEvent {
    pub item: Entity,
    pub person: Entity,
}
