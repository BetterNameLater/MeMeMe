use crate::items::components::cooldown::Cooldown;
use crate::items::components::is_activated::IsActivated;
use crate::player::events::rewind_event::RewindEvent;
use bevy::prelude::*;

/// Rewind items to theyre initial state
pub fn reset_level_items(
    mut commands: Commands,
    mut is_activated_query: Query<&mut IsActivated>,
    cooldown_query: Query<Entity, With<Cooldown<IsActivated>>>,
    mut rewind_event: EventReader<RewindEvent>,
) {
    if rewind_event.is_empty() {
        return;
    }
    rewind_event.clear();
    debug!("Rewind");
    for item in cooldown_query.iter() {
        commands.entity(item).remove::<Cooldown<IsActivated>>();
    }

    // reset items state
    for mut is_activated in &mut is_activated_query {
        // TODO check if the player is on a pressure plate
        // TODO check if a lever is set to `true` from the beginning ?
        is_activated.0 = false;
    }
}
