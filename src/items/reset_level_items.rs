use crate::items::components::cooldown::Cooldown;
use crate::items::components::is_activated::IsActivated;
use crate::items::components::people_on::PeopleOn;
use crate::player::events::rewind_event::RewindEvent;
use bevy::prelude::*;

/// Rewind items to theyre initial state
pub fn reset_level_items(
    mut commands: Commands,
    mut is_activated_query: Query<&mut IsActivated>,
    cooldown_query: Query<Entity, With<Cooldown<IsActivated>>>,
    mut rewind_event: EventReader<RewindEvent>,
    mut player_only_people_on_query: Query<&mut PeopleOn>,
) {
    if rewind_event.is_empty() {
        return;
    }
    rewind_event.clear();
    debug!("Rewind");
    for item in cooldown_query.iter() {
        commands.entity(item).remove::<Cooldown<IsActivated>>();
    }
    for mut people_on in player_only_people_on_query.iter_mut() {
        // TODO check count on initial
        people_on.0 = 0;
    }

    // reset items state
    for mut is_activated in &mut is_activated_query {
        // TODO check if the player is on a pressure plate
        // TODO check if a lever is set to `true` from the beginning ?
        is_activated.0 = false;
    }
}
