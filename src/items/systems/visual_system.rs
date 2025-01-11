use crate::items::primitive::is_activated::IsActivated;
use crate::items::primitive::is_usable::IsUsable;
use crate::items::primitive::item::{Item, ItemOutline, OutlineType};
use bevy::prelude::*;

pub fn visual_system(
    mut items_query: Query<(Option<&IsUsable>, Option<&IsActivated>), With<Item>>,
    mut outline_query: Query<(&ItemOutline, &mut Sprite)>,
) {
    for (outline, mut outline_sprite) in &mut outline_query {
        if let Ok((is_usable, is_activated)) = items_query.get_mut(outline.0) {
            match outline.1 {
                OutlineType::IsUsable => {
                    if is_usable.is_some() {
                        outline_sprite.color = bevy::color::palettes::css::GREEN.into();
                    } else {
                        outline_sprite.color = bevy::color::palettes::css::RED.into();
                    }
                }
                OutlineType::IsActivated => {
                    if let Some(is_activated) = is_activated {
                        if is_activated.0 {
                            outline_sprite.color = bevy::color::palettes::css::MIDNIGHT_BLUE.into();
                        } else {
                            outline_sprite.color = bevy::color::palettes::css::GRAY.into();
                        }
                    } else {
                        outline_sprite.custom_size = Some(Vec2::default());
                    }
                }
            }
        }
    }
}
