use crate::constantes::CELL_LENGTH;
use crate::items::bundle;
use crate::items::bundle::door_bundle::DoorBundle;
use crate::items::bundle::item_bundle::ItemBundle;
use crate::items::bundle::level_teleporter_bundle::LevelTeleporterBundle;
use crate::items::bundle::lever_bundle::LeverBundle;
use crate::items::bundle::pressure_plate_bundle::PressurePlateBundle;
use crate::items::bundle::pressure_plate_on_off_bundle::PressurePlateOnOffBundle;
use crate::items::bundle::teleporter_bundle::TeleporterBundle;
use crate::items::components::debug_name::DebugName;
use crate::items::components::dependencies::{Dependencies, Off, On};
use crate::items::components::enterable::EnterAble;
use crate::items::components::ghost_only::GhostOnly;
use crate::items::components::is_activated::IsActivated;
use crate::items::components::is_usable::IsUsable;
use crate::items::components::level_teleporter::LevelTeleporter;
use crate::items::components::people_on::PeopleOn;
use crate::items::components::player_only::PlayerOnly;
use crate::items::components::single_use::SingleUse;
use crate::items::components::teleporter::Teleporter;
use crate::items::components::toggle::Toggle;
use crate::map_parser::map_repr::{InteractionType, ObjectRepr, ObjectType};
use crate::math::vec2i::Vec2i;
use bevy::prelude::*;
use bevy::utils::HashMap;

pub fn populate_items(
    commands: &mut Commands,
    parent: Entity,
    objects: &HashMap<String, ObjectRepr>,
) -> HashMap<Vec2i, Entity> {
    let mut items_by_vec2i: HashMap<Vec2i, Entity> = HashMap::new();
    let mut items_by_name: HashMap<String, Entity> = HashMap::new();
    let size = CELL_LENGTH / 3.;

    for (key, object) in objects.iter() {
        let mut item: Option<Entity> = None;

        commands.entity(parent).with_children(|parent| {
            item = Some(
                parent
                    .spawn(ItemBundle {
                        is_usable: IsUsable,
                        is_activated: IsActivated(false),
                        debug_name: DebugName(key.clone()),
                    })
                    .id(),
            );
        });
        let item = item.unwrap();

        let position = Vec2i::new(
            object.position.x * CELL_LENGTH as i32,
            object.position.y * CELL_LENGTH as i32,
        );
        items_by_vec2i.insert(position, item);
        items_by_name.insert(key.clone(), item);

        match &object.object_type {
            ObjectType::PressurePlate => {
                commands.entity(item).insert(PressurePlateBundle {
                    // TODO check if player begins here
                    enterable: EnterAble,
                    people_on: PeopleOn(0),
                });
            }
            ObjectType::Teleporter { destination } => {
                commands.entity(item).insert(TeleporterBundle {
                    enterable: EnterAble,
                    teleporter: Teleporter(Vec2i::new(destination.x * 32, destination.y * 32)),
                });
            }
            ObjectType::Lever => {
                commands.entity(item).insert(LeverBundle {
                    toggle: Toggle::default(),
                });
            }
            ObjectType::Door => {
                commands.entity(item).insert(DoorBundle {});
            }
            ObjectType::LevelTeleporter { destination } => {
                commands.entity(item).insert(LevelTeleporterBundle {
                    enterable: EnterAble,
                    level_teleporter: LevelTeleporter(destination.clone()),
                });
            }
            ObjectType::PressurePlateOnOff => {
                commands.entity(item).insert(PressurePlateOnOffBundle {
                    enterable: EnterAble,
                    toggle_enter: Toggle::default(),
                });
            }
            ObjectType::Button => {
                commands
                    .entity(item)
                    .insert(bundle::button_bundle::ButtonBundle {
                        toggle_interact: Toggle::default(),
                    });
            }
        };

        let item_color = match object.object_type {
            ObjectType::PressurePlate => Color::GREEN,
            ObjectType::Teleporter { .. } => Color::GOLD,
            ObjectType::Lever => Color::YELLOW,
            ObjectType::Door => Color::MIDNIGHT_BLUE,
            ObjectType::LevelTeleporter { .. } => Color::ALICE_BLUE,
            ObjectType::PressurePlateOnOff => Color::AZURE,
            ObjectType::Button => Color::DARK_GRAY,
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
        let dependencies_on_entities: Vec<Entity> = object
            .depends_on
            .iter()
            .filter(|(_, on)| **on)
            .map(|(name, on)| *items_by_name.get(name).expect("Ce nom n'est pas défini !"))
            .collect();
        if !dependencies_on_entities.is_empty() {
            commands
                .entity(*item)
                .insert(Dependencies::<On>::new(dependencies_on_entities));
        }
        let dependencies_off_entities: Vec<Entity> = object
            .depends_on
            .iter()
            .filter(|(_, on)| !**on)
            .map(|(name, on)| *items_by_name.get(name).expect("Ce nom n'est pas défini !"))
            .collect();
        if !dependencies_off_entities.is_empty() {
            commands
                .entity(*item)
                .insert(Dependencies::<Off>::new(dependencies_off_entities));
        }
        println!("{:?} {:?} have now deps !", name, item)
    }

    items_by_vec2i
}
