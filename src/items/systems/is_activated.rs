use crate::items::dependencies::Dependencies;
use crate::items::is_usable::IsUsable;
use bevy::prelude::*;

/// Component representing the state of an item. Is it enable, can we interact with it ?
/// - Door
/// - ...
#[derive(Component)]
pub struct IsActivated(pub bool);

pub fn update_is_activated_system(
    mut commands: Commands,
    is_activated_query: Query<(Entity, &IsActivated)>,
    is_usable_and_dependencies_query: Query<(Entity, &Dependencies, Option<&IsUsable>)>,
) {
    for (is_usable_entity, is_usable_dependencies, is_usable) in &is_usable_and_dependencies_query {
        let my_dependencies = is_activated_query
            .iter()
            .filter(|(id, is_activated)| is_usable_dependencies.0.contains(id) && !is_activated.0);
        if my_dependencies.count() == 0 && is_usable.is_none() {
            commands.entity(is_usable_entity).insert(IsUsable);
        } else if let Some(_) = is_usable {
            commands.entity(is_usable_entity).remove::<IsUsable>();
        }
    }
}
