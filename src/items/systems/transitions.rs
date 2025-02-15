use crate::items::primitive::cooldown::Cooldown;
use crate::items::primitive::is_activated::IsActivated;
use crate::items::primitive::people_on::PeopleOn;
use bevy::prelude::*;

pub fn enter_rewind(
    commands: Commands,
    is_activated_query: Query<&mut IsActivated>,
    cooldown_query: Query<Entity, With<Cooldown>>,
    player_only_people_on_query: Query<&mut PeopleOn>,
) {
    reset_level_items(
        commands,
        is_activated_query,
        cooldown_query,
        player_only_people_on_query,
    );
}

/// Rewind items to theyre initial state
fn reset_level_items(
    mut commands: Commands,
    mut is_activated_query: Query<&mut IsActivated>,
    cooldown_query: Query<Entity, With<Cooldown>>,
    mut player_only_people_on_query: Query<&mut PeopleOn>,
) {
    debug!("Reset level itmes");
    for item in cooldown_query.iter() {
        commands.entity(item).remove::<Cooldown>();
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
