use crate::items::primitive::enterable::EnterAble;
use bevy::prelude::Component;

/// Represent a LevelTeleporter item
/// The LevelTeleporter call a state change to an other level when the player enter it
/// Il t'amène dans un autre niveau, via le nom.
#[derive(Component, Default)]
#[require(EnterAble)]
pub struct LevelTeleporter(pub String);
