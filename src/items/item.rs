use bevy::core::Name;
use bevy::prelude::{Component, Entity, Sprite};

/// Shared properties between all items
#[derive(Component)]
#[require(Name, Sprite)]
pub struct Item;

#[derive(Component)]
pub struct ItemOutline(pub Entity, pub OutlineType);

pub enum OutlineType {
    IsUsable,
    IsActivated,
}
