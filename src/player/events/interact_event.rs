use crate::player::components::person::Person;
use bevy::prelude::*;
use std::marker::PhantomData;

#[derive(Event, Debug)]
pub struct InteractEvent<T: Person> {
    pub person: Entity,
    pub item: Entity,
    pub person_marker: PhantomData<T>,
}

impl<T: Person> InteractEvent<T> {
    pub fn new(person: Entity, item: Entity) -> Self {
        Self {
            person,
            item,
            person_marker: PhantomData,
        }
    }
}
