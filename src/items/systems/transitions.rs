use crate::items::primitive::is_activated::IsActivated;
use crate::items::primitive::people_on::PeopleOn;
use bevy::prelude::*;

pub fn enter_rewind(
    is_activated_query: Query<&mut IsActivated>,
    player_only_people_on_query: Query<&mut PeopleOn>,
) {
    reset_level_items(is_activated_query, player_only_people_on_query);
}

/// Rewind items to theyre initial state
fn reset_level_items(
    mut is_activated_query: Query<&mut IsActivated>,
    mut player_only_people_on_query: Query<&mut PeopleOn>,
) {
    debug!("Reset level itmes");
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
