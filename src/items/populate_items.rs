use crate::constantes::{CELL_LENGTH, ITEMS_Z};
use crate::items::button::Button as ButtonItem;
use crate::items::door::Door;
use crate::items::item::{ItemBundle, ItemOutline, OutlineType};
use crate::items::level_teleporter::LevelTeleporter;
use crate::items::lever::Lever;
use crate::items::pressure_plate::PressurePlate;
use crate::items::pressure_plate_on_off::PressurePlateOnOff;
use crate::items::primitive::dependencies::{Dependencies, Off, On};
use crate::items::primitive::ghost_only::GhostOnly;
use crate::items::primitive::killing::Killing;
use crate::items::primitive::player_only::PlayerOnly;
use crate::items::primitive::single_use::SingleUse;
use crate::items::primitive::start_timer::StartTimer;
use crate::items::primitive::teleporter::Teleporter;
use crate::items::teleporter::TeleporterBundle;
use crate::map_parser::map_repr::{InteractionType, ObjectRepr, ObjectType};
use crate::math::vec2i::Vec2i;
use bevy::prelude::*;
use std::collections::HashMap;

pub fn populate_items(
    commands: &mut Commands,
    parent: Entity,
    objects: &HashMap<String, ObjectRepr>,
) -> HashMap<Vec2i, Entity> {
    let mut items_by_vec2i: HashMap<Vec2i, Entity> = HashMap::new();
    let mut items_by_name: HashMap<String, Entity> = HashMap::new();

    for (key, object) in objects.iter() {
        let position = Vec2i::new(
            object.position.x * CELL_LENGTH as i32,
            object.position.y * CELL_LENGTH as i32,
        );
        let item = commands
            .spawn((
                ItemBundle::new(key, &object.object_type),
                position.to_transform(ITEMS_Z),
            ))
            .id();

        #[cfg(debug_assertions)]
        add_debug_outlines(commands, item);
        commands.entity(parent).add_child(item);
        items_by_vec2i.insert(position, item);
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
                commands.entity(item).insert(TeleporterBundle {
                    teleporter: Teleporter(Vec2i::new(destination.x * 32, destination.y * 32)),
                    ..default()
                });
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
            ObjectType::PressurePlateOnOff => {
                commands.entity(item).insert(PressurePlateOnOff);
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

        if object.killing {
            commands.entity(item).insert(Killing);
        }

        if let Some(start_timer) = object.start_timer {
            commands.entity(item).insert(StartTimer(Timer::from_seconds(
                start_timer,
                TimerMode::Once,
            )));
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
    items_by_vec2i
}

#[cfg(debug_assertions)]
fn add_debug_outlines(commands: &mut Commands, item: Entity) {
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
