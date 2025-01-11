use crate::items::primitive::dependencies::{Dependencies, Off, On};
use crate::items::primitive::is_activated::IsActivated;
use crate::items::primitive::is_usable::IsUsable;
use crate::items::primitive::item::Item;
use bevy::prelude::*;

pub fn update_is_usable_system(
    mut commands: Commands,
    entity_is_activated_query: Query<(Entity, &IsActivated)>,
    entity_dependencies_query: Query<(Entity, Option<&Dependencies<On>>), With<Item>>,
) {
    for (item, some_dependencies) in &entity_dependencies_query {
        if let Some(dependencies) = some_dependencies {
            let one_is_not_activated = entity_is_activated_query
                .iter()
                .any(|(id, is_activated)| dependencies.0.contains(&id) && !is_activated.0);
            if one_is_not_activated {
                commands.entity(item).remove::<IsUsable>();
            } else {
                commands.entity(item).insert(IsUsable);
            }
        } else {
            commands.entity(item).insert(IsUsable);
        }
    }
}

pub fn update_is_unusable_system(
    mut commands: Commands,
    is_activated_query: Query<(Entity, &IsActivated)>,
    is_usable_and_dependencies_query: Query<(
        Entity,
        &Dependencies<Off>,
        Option<&Dependencies<On>>,
    )>,
) {
    for (is_usable_entity, is_unusable_dependencies, on_dependencies) in
        &is_usable_and_dependencies_query
    {
        let has_on_dependencies = on_dependencies.is_some();
        let one_is_activated = is_activated_query
            .iter()
            .any(|(id, is_activated)| is_unusable_dependencies.0.contains(&id) && is_activated.0);
        if !has_on_dependencies {
            if one_is_activated {
                commands.entity(is_usable_entity).remove::<IsUsable>();
            } else {
                commands.entity(is_usable_entity).insert(IsUsable);
            }
        } else if one_is_activated {
            // si update_is_usable_system a ajouté `IsUsable`
            // on peut l'enlever
            commands.entity(is_usable_entity).remove::<IsUsable>();
        }
    }
}
