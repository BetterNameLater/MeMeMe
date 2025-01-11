use crate::math::vec2i::Vec2i;
use bevy::prelude::*;
use std::marker::PhantomData;

#[derive(Event, Debug)]
pub struct InteractEvent<T> {
    pub _pos: Vec2i,
    pub person: Entity,
    pub item: Entity,
    pub marker: PhantomData<T>,
}

impl<T> InteractEvent<T> {
    pub fn new(pos: Vec2i, person: Entity, item: Entity) -> Self {
        Self {
            _pos: pos,
            person,
            item,
            marker: Default::default(),
        }
    }
}
