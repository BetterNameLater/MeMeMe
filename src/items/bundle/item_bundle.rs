use crate::constantes::CELL_LENGTH;
use crate::map_parser::map_repr::ObjectType;
use bevy::core::Name;
use bevy::math::Vec2;
use bevy::prelude::{default, Bundle, Component, Entity, Sprite};

/// Shared properties between all items
#[derive(Bundle)]
pub struct ItemBundle {
    name: Name,
    sprite: Sprite,
    marker: Item,
}

#[derive(Component)]
pub struct Item;

#[derive(Component)]
pub struct ItemOutline(pub Entity, pub OutlineType);

pub enum OutlineType {
    IsUsable,
    IsActivated,
}

impl ItemBundle {
    pub fn new(debug_name: &str, object_type: &ObjectType) -> Self {
        let size = CELL_LENGTH / 3.;

        let color = match object_type {
            ObjectType::PressurePlate => bevy::color::palettes::css::GREEN.into(),
            ObjectType::Teleporter { .. } => bevy::color::palettes::css::GOLD.into(),
            ObjectType::Lever => bevy::color::palettes::css::YELLOW.into(),
            ObjectType::Door => bevy::color::palettes::css::MIDNIGHT_BLUE.into(),
            ObjectType::LevelTeleporter { .. } => bevy::color::palettes::css::ALICE_BLUE.into(),
            ObjectType::PressurePlateOnOff => bevy::color::palettes::css::AZURE.into(),
            ObjectType::Button => bevy::color::palettes::css::DARK_GRAY.into(),
        };

        ItemBundle {
            name: Name::new(debug_name.to_string()),
            sprite: Sprite {
                color,
                custom_size: Some(Vec2::new(size, size)),
                ..default()
            },
            marker: Item,
        }
    }
}
