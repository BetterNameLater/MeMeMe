use crate::items::primitive::item::Item;
use bevy::prelude::Component;

#[derive(Component)]
#[require(Item)]
pub struct Door;
