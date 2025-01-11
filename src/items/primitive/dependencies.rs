use bevy::prelude::{Component, Entity};
use std::marker::PhantomData;

/// List all the dependencies of an item
/// Permets de définir de quelles entités un item dépend pour être utilisable.
#[derive(Component, Default)]
pub struct Dependencies<T: DependenciesType>(pub Vec<Entity>, PhantomData<T>);

impl<T: DependenciesType> Dependencies<T> {
    pub fn new(entities: Vec<Entity>) -> Self {
        Self(entities, PhantomData)
    }
}

pub trait DependenciesType {}

#[derive(Default)]
pub struct On;
impl DependenciesType for On {}

#[derive(Default)]
pub struct Off;
impl DependenciesType for Off {}
