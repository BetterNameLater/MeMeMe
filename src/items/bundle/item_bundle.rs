use crate::constantes::CELL_LENGTH;
use crate::items::components::is_activated::IsActivated;
use crate::items::components::is_usable::IsUsable;
use crate::map_parser::map_repr::ObjectType;
use crate::math::vec2i::Vec2i;
use bevy::core::Name;
use bevy::math::Vec2;
use bevy::prelude::{default, Bundle, Color, Sprite};
use bevy::sprite::SpriteBundle;

/// Shared properties between all items
#[derive(Bundle)]
pub struct ItemBundle {
    is_activated: IsActivated,
    is_usable: IsUsable,
    name: Name,
    sprite_bundle: SpriteBundle,
}

impl ItemBundle {
    pub fn new(debug_name: &str, position: Vec2i, object_type: &ObjectType) -> Self {
        let size = CELL_LENGTH / 3.;

        let color = match object_type {
            ObjectType::PressurePlate => Color::GREEN,
            ObjectType::Teleporter { .. } => Color::GOLD,
            ObjectType::Lever => Color::YELLOW,
            ObjectType::Door => Color::MIDNIGHT_BLUE,
            ObjectType::LevelTeleporter { .. } => Color::ALICE_BLUE,
            ObjectType::PressurePlateOnOff => Color::AZURE,
            ObjectType::Button => Color::DARK_GRAY,
        };

        ItemBundle {
            is_usable: IsUsable,
            is_activated: IsActivated(false),
            name: Name::new(debug_name.to_string()),
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::new(size, size)),
                    ..default()
                },
                transform: position.to_transform(1),
                ..default()
            },
        }
    }
}
