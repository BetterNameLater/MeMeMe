use crate::items::ghost_only::GhostOnly;
use crate::items::player_only::PlayerOnly;
use crate::player::{Ghost, GhostNewPositionEvent, Player, PlayerNewPositionEvent};
use bevy::prelude::*;

/// The number of [`Ghost`] and [`Player`] on this position
#[derive(Component)]
pub struct PeopleOn(pub usize);

pub fn count_people_on_system(
    player_new_position_event: EventReader<PlayerNewPositionEvent>,
    ghost_new_position_event: EventReader<GhostNewPositionEvent>,
    mut people_on_query: Query<&mut PeopleOn, (Without<PlayerOnly>, Without<GhostOnly>)>,
) {
}

pub fn count_people_on_ghost_only_system(
    ghost_new_position_event: EventReader<GhostNewPositionEvent>,
    mut people_on_query: Query<&mut PeopleOn, (With<GhostOnly>, Without<Player>)>,
) {
}

pub fn count_people_on_player_only_system(
    player_new_position_event: EventReader<PlayerNewPositionEvent>,
    mut people_on_query: Query<&mut PeopleOn, (With<PlayerOnly>, Without<Ghost>)>,
) {
}
