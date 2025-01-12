use crate::items::primitive::colliding::Colliding;
use crate::items::primitive::item::Item;
use bevy::prelude::Component;

#[derive(Component)]
#[require(Item, Colliding)]
pub struct Door;
