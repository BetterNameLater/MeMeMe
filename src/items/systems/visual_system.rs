use crate::items::bundle::item_bundle::{Item, ItemOutline, OutlineType};
use crate::items::components::is_activated::IsActivated;
use crate::items::components::is_usable::IsUsable;
use bevy::prelude::*;

pub fn visual_system(
    mut items_query: Query<(Entity, Option<&IsUsable>, Option<&IsActivated>), With<Item>>,
    mut outline_query: Query<(&ItemOutline, &mut Sprite)>,
) {
    for (item, is_usable, is_activated) in &mut items_query {
        for (outline, mut outline_sprite) in &mut outline_query {
            if outline.0 == item {
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
                            outline_sprite.color = Color::WHITE;
                        }
                    }
                }
            }
        }
    }
}
