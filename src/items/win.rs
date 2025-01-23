use crate::items::primitive::enterable::EnterAble;
use crate::items::primitive::item::Item;
use bevy::prelude::Component;

#[derive(Component, Default)]
#[require(Item, EnterAble)]
pub struct Win;
