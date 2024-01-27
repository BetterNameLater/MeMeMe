use crate::math::vec2i::Vec2i;
use bevy::prelude::*;
use std::marker::PhantomData;

#[derive(Event)]
pub struct InteractEvent<T> {
    pub pos: Vec2i,
    pub entity: Entity,
    pub marker: PhantomData<T>,
}

impl<T> InteractEvent<T> {
    pub fn new(pos: Vec2i, entity: Entity) -> Self {
        Self {
            pos,
            entity,
            marker: Default::default(),
        }
    }
}
