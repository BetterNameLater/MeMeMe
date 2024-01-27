use crate::items::components::dependencies::{Dependencies, Off, On};
use crate::items::components::is_activated::IsActivated;
use crate::items::components::is_usable::IsUsable;
use bevy::prelude::*;

pub fn update_is_usable_system(
    mut commands: Commands,
    is_activated_query: Query<(Entity, &IsActivated)>,
    is_usable_and_dependencies_query: Query<(Entity, &Dependencies<On>, Option<&IsUsable>)>,
) {
    for (is_usable_entity, is_usable_dependencies, is_usable) in &is_usable_and_dependencies_query {
        let my_dependencies_deactivated = is_activated_query
            .iter()
            .filter(|(id, is_activated)| is_usable_dependencies.0.contains(id) && !is_activated.0);
        // all are activated
        if my_dependencies_deactivated.count() == 0 && is_usable.is_none() {
            commands.entity(is_usable_entity).insert(IsUsable);
        // one is unactivated
        } else {
            commands.entity(is_usable_entity).remove::<IsUsable>();
        }
    }
}

pub fn update_is_unusable_system(
    mut commands: Commands,
    is_activated_query: Query<(Entity, &IsActivated)>,
    is_usable_and_dependencies_query: Query<(
        Entity,
        &Dependencies<Off>,
        Option<&IsUsable>,
        Option<&Dependencies<On>>,
    )>,
) {
    for (is_usable_entity, is_unusable_dependencies, is_usable, u_deps) in
        &is_usable_and_dependencies_query
    {
        let my_dependencies_activated = is_activated_query
            .iter()
            .filter(|(id, is_activated)| is_unusable_dependencies.0.contains(id) && is_activated.0);
        let count = my_dependencies_activated.count();

        // si ça ne depend pas de Dependencies<On> -> je fais mon histoire tout seul
        if u_deps.is_none() {
            // all are deactivated
            if count == 0 && is_usable.is_none() {
                commands.entity(is_usable_entity).insert(IsUsable);
                // one is activated
            } else {
                commands.entity(is_usable_entity).remove::<IsUsable>();
            }
        } else if count != 0 {
            // si update_is_usable_system a ajouté `IsUsable`
            // on peut l'enlever
            commands.entity(is_usable_entity).remove::<IsUsable>();
        }
    }
}
