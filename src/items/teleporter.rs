use crate::items::primitive::enterable::EnterAble;
use crate::items::primitive::item::Item;
use crate::math::vec2i::Vec2i;
use bevy::prelude::Component;

/// Represent a Teleporter item
/// When a person enter the teleporter, it moves the person to the destination
#[derive(Component, Default)]
#[require(Item, EnterAble)]
pub struct Teleporter(pub Vec2i);
