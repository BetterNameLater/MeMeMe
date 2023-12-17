use crate::constantes::{CELL_LENGTH, PLAYER_START_TRANSFORM};
use crate::items::dependencies::Dependencies;
use crate::items::ghost_only::GhostOnly;
use crate::items::is_usable::IsUsable;
use crate::items::people_on::PeopleOn;
use crate::items::player_only::PlayerOnly;
use crate::map_parser::map_repr::{ObjectRepr, ObjectType};
use crate::math::vec2i::Vec2i;
use bevy::prelude::*;
use bevy::utils::HashMap;

use super::player_only::SingleUse;
use super::teleport::Teleporter;

#[derive(Component)]
pub struct Item;

pub fn populate_items(
    mut commands: &mut Commands,
    objects: &HashMap<String, ObjectRepr>,
) -> HashMap<Vec2i, Entity> {
    let mut items_by_vec2i: HashMap<Vec2i, Entity> = HashMap::new();
    let mut items_by_name: HashMap<String, Entity> = HashMap::new();
    let size = CELL_LENGTH / 3.;

    for (key, object) in objects.iter() {
        let item = commands.spawn(Item).id();
        let position = Vec2i::new(
            object.position.x * CELL_LENGTH as i32,
            object.position.y * CELL_LENGTH as i32,
        );
        items_by_vec2i.insert(position, item.clone());
        items_by_name.insert(key.clone(), item.clone());

        match object.object_type {
            ObjectType::PressurePlate => {
                println!("ma pressure plate");
                commands.entity(item).insert(PeopleOn(0));
                commands.entity(item).insert(IsUsable(false));
                commands.entity(item).insert(SpriteBundle {
                    sprite: Sprite {
                        color: Color::LIME_GREEN,
                        custom_size: Some(Vec2::new(size, size)),
                        ..default()
                    },
                    // TODO in a parameter
                    transform: position.to_initial_map_pos(1),
                    ..default()
                });
            }
            ObjectType::Teleporter => {
                println!("{:?}", object.destination);
                if let Some(destination) = object.destination {
                    commands.entity(item).insert(Teleporter(Vec2i::new(
                        destination.x * 32,
                        destination.y * 32,
                    )));
                }
                commands.entity(item).insert(SpriteBundle {
                    sprite: Sprite {
                        color: Color::BLACK,
                        custom_size: Some(Vec2::new(size, size)),
                        ..default()
                    },
                    // TODO in a parameter
                    transform: position.to_initial_map_pos(1),
                    ..default()
                });
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

    for (name, item) in items_by_name.iter() {
        let object = objects.get(name).unwrap();
        if object.depends_on.is_empty() {
            continue;
        }
        let deps_entities: Vec<Entity> = object
            .depends_on
            .iter()
            .map(|name| {
                items_by_name
                    .get(name)
                    .expect("Ce nom n'est pas défini !")
                    .clone()
            })
            .collect();
        commands
            .entity(item.clone())
            .insert(Dependencies(deps_entities));
    }
    return items_by_vec2i;
}
