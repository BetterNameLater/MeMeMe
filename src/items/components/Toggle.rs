use bevy::prelude::Component;
use std::marker::PhantomData;

#[derive(Component, Default)]
pub struct Toggle<T: ToggleType> {
    marker: PhantomData<T>,
}

pub trait ToggleType: Default {}

#[derive(Default)]
pub struct Interact;
impl ToggleType for Interact {}

#[derive(Default)]
pub struct Enter;
impl ToggleType for Enter {}
