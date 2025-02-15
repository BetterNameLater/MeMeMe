use crate::constantes::{CELL_LENGTH, ITEMS_Z};
use crate::items::button::Button as ButtonItem;
use crate::items::door::Door;
use crate::items::interaction_type::ghost_only::GhostOnly;
use crate::items::interaction_type::player_only::PlayerOnly;
use crate::items::level_teleporter::LevelTeleporter;
use crate::items::lever::Lever;
use crate::items::pressure_plate::PressurePlate;
use crate::items::primitive::dependencies::{Dependencies, Off, On};
use crate::items::primitive::item::{ItemOutline, OutlineType};
use crate::items::primitive::single_use::SingleUse;
use crate::items::teleporter::Teleporter;
use bevy::prelude::*;
use level_parser::{InteractionType, ObjectRepr, ObjectType};
use std::collections::HashMap;

const ITEM_SIZE: f32 = CELL_LENGTH / 3.;

pub fn populate_items(
    commands: &mut Commands,
    parent: Entity,
    objects: &HashMap<String, ObjectRepr>,
) {
    let mut items_by_name: HashMap<String, Entity> = HashMap::new();

    for (key, object) in objects.iter() {
        let position = IVec2::new(
            object.position.x * CELL_LENGTH as i32,
            object.position.y * CELL_LENGTH as i32,
        );
        let item = commands
            .spawn((
                Name::new(key.clone()),
                Sprite {
                    color: match object.object_type {
                        ObjectType::PressurePlate => bevy::color::palettes::css::GREEN.into(),
                        ObjectType::Teleporter { .. } => bevy::color::palettes::css::GOLD.into(),
                        ObjectType::Lever => bevy::color::palettes::css::YELLOW.into(),
                        ObjectType::Door => bevy::color::palettes::css::MIDNIGHT_BLUE.into(),
                        ObjectType::LevelTeleporter { .. } => {
                            bevy::color::palettes::css::ALICE_BLUE.into()
                        }
                        ObjectType::Button => bevy::color::palettes::css::DARK_GRAY.into(),
                    },
                    custom_size: Some(Vec2::new(ITEM_SIZE, ITEM_SIZE)),
                    ..default()
                },
                Transform::from_translation(position.as_vec2().extend(ITEMS_Z)),
            ))
            .id();

        add_item_state_outlines(commands, item);
        commands.entity(parent).add_child(item);
        items_by_name.insert(key.clone(), item);

        debug!(
            "Spawning a {:?} named {:?} on position {:?} as {:?}",
            object.object_type, key, position, item
        );

        match &object.object_type {
            ObjectType::PressurePlate => {
                // TODO check if player begins here
                commands.entity(item).insert(PressurePlate);
            }
            ObjectType::Teleporter { destination } => {
                commands.entity(item).insert(Teleporter(IVec2::new(
                    destination.x * 32,
                    destination.y * 32,
                )));
            }
            ObjectType::Lever => {
                commands.entity(item).insert(Lever);
            }
            ObjectType::Door => {
                commands.entity(item).insert(Door);
            }
            ObjectType::LevelTeleporter { destination } => {
                commands
                    .entity(item)
                    .insert(LevelTeleporter(destination.clone()));
            }
            ObjectType::Button => {
                commands.entity(item).insert(ButtonItem);
            }
        };

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
            .map(|(name, _)| *items_by_name.get(name).expect("Ce nom n'est pas défini !"))
            .collect();
        if !dependencies_on_entities.is_empty() {
            commands
                .entity(*item)
                .insert(Dependencies::<On>::new(dependencies_on_entities));
            debug!("Item {:?} ({:?}) have `On` dependencies !", name, item);
        }
        let dependencies_off_entities: Vec<Entity> = object
            .depends_on
            .iter()
            .filter(|(_, on)| !**on)
            .map(|(name, _)| *items_by_name.get(name).expect("Ce nom n'est pas défini !"))
            .collect();
        if !dependencies_off_entities.is_empty() {
            commands
                .entity(*item)
                .insert(Dependencies::<Off>::new(dependencies_off_entities));
            debug!("Item {:?} ({:?}) have `Off` dependencies !", name, item);
        }
    }

    trace!("Items has been populated");
}

/// Add a visuale marker indicating state off the item
fn add_item_state_outlines(commands: &mut Commands, item: Entity) {
    const DISTANCE: f32 = 8.;

    let outline_usable = commands
        .spawn((
            ItemOutline(item, OutlineType::IsUsable),
            Transform::from_xyz(-DISTANCE, 0., -1.),
            Sprite {
                custom_size: Some(Vec2::new(5., 5.)),
                ..default()
            },
        ))
        .id();

    let outline_activated = commands
        .spawn((
            ItemOutline(item, OutlineType::IsActivated),
            Transform::from_xyz(DISTANCE, 0., -1.),
            Sprite {
                custom_size: Some(Vec2::new(5., 5.)),
                ..default()
            },
        ))
        .id();

    commands
        .entity(item)
        .add_children(&[outline_usable, outline_activated]);
}
