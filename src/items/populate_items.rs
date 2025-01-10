use crate::constantes::CELL_LENGTH;
use crate::items::bundle;
use crate::items::bundle::door_bundle::DoorBundle;
use crate::items::bundle::item_bundle::{ItemBundle, ItemOutline, OutlineType};
use crate::items::bundle::level_teleporter_bundle::LevelTeleporterBundle;
use crate::items::bundle::lever_bundle::LeverBundle;
use crate::items::bundle::pressure_plate_bundle::PressurePlateBundle;
use crate::items::bundle::pressure_plate_on_off_bundle::PressurePlateOnOffBundle;
use crate::items::bundle::teleporter_bundle::TeleporterBundle;
use crate::items::components::dependencies::{Dependencies, Off, On};
use crate::items::components::ghost_only::GhostOnly;
use crate::items::components::killing::Killing;
use crate::items::components::level_teleporter::LevelTeleporter;
use crate::items::components::player_only::PlayerOnly;
use crate::items::components::single_use::SingleUse;
use crate::items::components::start_timer::StartTimer;
use crate::items::components::teleporter::Teleporter;
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
            .spawn(ItemBundle::new(key, position, &object.object_type))
            .id();

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
                commands.entity(item).insert(PressurePlateBundle::default());
            }
            ObjectType::Teleporter { destination } => {
                commands.entity(item).insert(TeleporterBundle {
                    teleporter: Teleporter(Vec2i::new(destination.x * 32, destination.y * 32)),
                    ..default()
                });
            }
            ObjectType::Lever => {
                commands.entity(item).insert(LeverBundle::default());
            }
            ObjectType::Door => {
                commands.entity(item).insert(DoorBundle::default());
            }
            ObjectType::LevelTeleporter { destination } => {
                commands.entity(item).insert(LevelTeleporterBundle {
                    level_teleporter: LevelTeleporter(destination.clone()),
                    ..default()
                });
            }
            ObjectType::PressurePlateOnOff => {
                commands
                    .entity(item)
                    .insert(PressurePlateOnOffBundle::default());
            }
            ObjectType::Button => {
                commands
                    .entity(item)
                    .insert(bundle::button_bundle::ButtonBundle::default());
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

fn add_debug_outlines(commands: &mut Commands, item: Entity) {
    const DISTANCE: f32 = 8.;
    let mut s = SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(5., 5.)),
            ..default()
        },
        ..default()
    };

    s.transform.translation.z = -1.;
    s.transform.translation.x = -DISTANCE;
    let outline_usable = commands
        .spawn((ItemOutline(item, OutlineType::IsUsable), s.clone()))
        .id();

    s.transform.translation.z = -2.;
    s.transform.translation.x = DISTANCE;
    let outline_activated = commands
        .spawn((ItemOutline(item, OutlineType::IsActivated), s))
        .id();

    commands.entity(item).add_child(outline_usable);
    commands.entity(item).add_child(outline_activated);
}
