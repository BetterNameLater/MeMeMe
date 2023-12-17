use crate::player::PlayerNewPositionEvent;
use bevy::prelude::*;

/// The number of [`Ghost`] and [`Player`] on this position
#[derive(Component)]
pub struct PeopleOn(pub usize);

pub fn count_people_on_system(
    /*
    TODO
    - on player move event
    - on ghost move event

     */
    on_enter_event: EventReader<PlayerNewPositionEvent>,

    mut people_on_query: Query<&mut PeopleOn>,
) {
}
