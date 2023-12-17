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
    dependencies: Query<(Entity, &IsActivated)>,
    usables_query: Query<(Entity, &Dependencies, Option<&IsUsable>)>,
) {
    for (entity, deps_list, current_state) in &usables_query {
        let my_dependencies = dependencies
            .iter()
            .filter(|(ent, a)| deps_list.0.contains(ent) && !a.0);
        if my_dependencies.count() == 0 {
			if current_state.is_none() {
            	commands.entity(entity).insert(IsUsable);
			}
        } else if let Some(_) = current_state {
			commands.entity(entity).remove::<IsUsable>();
        }
    }
}
