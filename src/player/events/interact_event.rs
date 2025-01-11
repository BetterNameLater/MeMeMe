use crate::{math::vec2i::Vec2i, player::components::person::Person};
use bevy::prelude::*;
use std::marker::PhantomData;

#[derive(Event, Debug)]
pub struct InteractEvent<T: Person> {
    pub _pos: Vec2i,
    pub person: Entity,
    pub item: Entity,
    pub person_marker: PhantomData<T>,
}

impl<T: Person> InteractEvent<T> {
    pub fn new(pos: Vec2i, person: Entity, item: Entity) -> Self {
        Self {
            _pos: pos,
            person,
            item,
            person_marker: PhantomData,
        }
    }
}
