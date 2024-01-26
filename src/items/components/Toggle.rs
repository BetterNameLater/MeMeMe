use bevy::prelude::Component;
use std::marker::PhantomData;

#[derive(Default)]
pub struct Interact;

#[derive(Default)]
pub struct Enter;

#[derive(Component, Default)]
pub struct Toggle<T> {
    pub marker: PhantomData<T>,
}
