use crate::items::bundle::item_bundle::{Item, ItemOutline, OutlineType};
use crate::items::components::is_activated::IsActivated;
use crate::items::components::is_usable::IsUsable;
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
                        outline_sprite.color = Color::GREEN;
                    } else {
                        outline_sprite.color = Color::RED;
                    }
                }
                OutlineType::IsActivated => {
                    if let Some(is_activated) = is_activated {
                        if is_activated.0 {
                            outline_sprite.color = Color::MIDNIGHT_BLUE;
                        } else {
                            outline_sprite.color = Color::GRAY;
                        }
                    } else {
                        outline_sprite.custom_size = Some(Vec2::default());
                    }
                }
            }
        }
    }
}
