use crate::constantes::CELL_LENGTH;
use crate::items::debug_name::DebugName;
use crate::items::dependencies::Dependencies;
use crate::items::enterable::EnterAble;
use crate::items::ghost_only::GhostOnly;
use crate::items::is_usable::IsUsable;
use crate::items::level_teleporter::LevelTeleporter;
use crate::items::player_only::{PlayerOnly, SingleUse};
use crate::items::systems::is_activated::IsActivated;
use crate::items::systems::people_on::PeopleOn;
use crate::items::systems::teleport::Teleporter;
use crate::items::systems::toggle::ToggleInteract;
use crate::map_parser::map_repr::{InteractionType, ObjectRepr, ObjectType};
use crate::math::vec2i::Vec2i;
use bevy::prelude::*;
use bevy::utils::HashMap;

pub fn populate_items(
    commands: &mut Commands,
    objects: &HashMap<String, ObjectRepr>,
) -> HashMap<Vec2i, Entity> {
    let mut items_by_vec2i: HashMap<Vec2i, Entity> = HashMap::new();
    let mut items_by_name: HashMap<String, Entity> = HashMap::new();
    let size = CELL_LENGTH / 3.;

    for (key, object) in objects.iter() {
        let item = commands
            .spawn((IsActivated(false), IsUsable, DebugName(key.clone())))
            .id();
        let position = Vec2i::new(
            object.position.x * CELL_LENGTH as i32,
            object.position.y * CELL_LENGTH as i32,
        );
        items_by_vec2i.insert(position, item);
        items_by_name.insert(key.clone(), item);

        match &object.object_type {
            ObjectType::PressurePlate => {
                commands.entity(item).insert(PeopleOn(0));
            }
            ObjectType::Teleporter { destination } => {
                commands.entity(item).insert(Teleporter(Vec2i::new(
                    destination.x * 32,
                    destination.y * 32,
                )));
            }
            ObjectType::Lever => {
                commands.entity(item).insert(ToggleInteract);
            }
            ObjectType::Door => {}
            ObjectType::LevelTeleporter { destination } => {
                commands.entity(item).insert(EnterAble);
                commands
                    .entity(item)
                    .insert(LevelTeleporter(destination.clone()));
            }
        };

        let item_color = match object.object_type {
            ObjectType::PressurePlate => Color::GREEN,
            ObjectType::Teleporter { .. } => Color::BLUE,
            ObjectType::Lever => Color::YELLOW,
            ObjectType::Door => Color::MIDNIGHT_BLUE,
            ObjectType::LevelTeleporter { .. } => Color::BISQUE,
        };

        commands.entity(item).insert(SpriteBundle {
            sprite: Sprite {
                color: item_color,
                custom_size: Some(Vec2::new(size, size)),
                ..default()
            },
            // TODO in a parameter
            transform: position.to_initial_map_pos(1),
            ..default()
        });

        match object.interaction_type {
            InteractionType::GhostOnly => {
                commands.entity(item).insert(GhostOnly);
            }
            InteractionType::PlayerOnly => {
                commands.entity(item).insert(PlayerOnly);
            }
            InteractionType::All => {}
        }

        if object.single_use {
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
            .map(|name| *items_by_name.get(name).expect("Ce nom n'est pas défini !"))
            .collect();
        commands.entity(*item).insert(Dependencies(deps_entities));
    }
    items_by_vec2i
}
