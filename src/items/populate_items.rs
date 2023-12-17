use crate::constantes::{CELL_LENGTH, PLAYER_START_TRANSFORM};
use crate::items::ghost_only::GhostOnly;
use crate::items::is_on::IsOn;
use crate::items::people_on::PeopleOn;
use crate::items::player_only::PlayerOnly;
use crate::map_parser::map_repr::{ObjectRepr, ObjectType};
use crate::math::vec2i::Vec2i;
use bevy::prelude::*;
use bevy::utils::HashMap;

use super::player_only::SingleUse;

#[derive(Component)]
pub struct Item;

pub fn populate_items(
    mut commands: Commands,
    objects: &HashMap<String, ObjectRepr>,
) -> HashMap<Vec2i, Entity> {
    let mut items: HashMap<Vec2i, Entity> = HashMap::default();

    for (key, object) in objects.iter() {
        let item = commands.spawn(Item).id();
        items.insert(object.position.into(), item.clone());

        match object.object_type {
            ObjectType::PressurePlate => {
                commands.entity(item).insert(PeopleOn(0));
                commands.entity(item).insert(IsOn(false));
            }
            _ => {}
        }

        if (object.ghost_only) {
            commands.entity(item).insert(GhostOnly);
        } else if (object.player_only) {
            commands.entity(item).insert(PlayerOnly);
        } else if (object.single_use) {
            commands.entity(item).insert(SingleUse);
        }
    }
    return items;
}
