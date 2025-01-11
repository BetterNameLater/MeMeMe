use crate::items::primitive::enterable::EnterAble;
use crate::items::primitive::item::Item;
use bevy::prelude::Component;

/// Represent a LevelTeleporter item
/// The LevelTeleporter call a state change to an other level when the player enter it
/// Il t'amène dans un autre niveau, via le nom.
#[derive(Component, Default)]
#[require(Item, EnterAble)]
pub struct LevelTeleporter(pub String);
