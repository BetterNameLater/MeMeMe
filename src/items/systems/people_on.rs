use crate::items::is_usable::IsUsable;

use crate::items::systems::is_activated::IsActivated;
use crate::map::{Map, ObjectMap};

use crate::player::events::NewPositionEvent;

use bevy::prelude::*;

/// The number of [`Ghost`] and [`Player`] on this position
#[derive(Component)]
pub struct PeopleOn(pub usize);

pub fn count_people_on_system<W: Component, E: NewPositionEvent>(
    mut player_new_position_event: EventReader<E>,
    mut player_only_people_on_query: Query<
        (&mut IsActivated, &mut PeopleOn),
        (With<IsUsable>, Without<W>),
    >,
    object_map_query: Query<&Map, With<ObjectMap>>,
) {
    let object_map = object_map_query.single();
    for new_position_event in player_new_position_event.read() {
        // println!("count_people_on_system");
        if let Some(&leaved_cell) = object_map.cells.get(&new_position_event.get_before()) {
            if let Ok((mut is_activated, mut people_on)) =
                player_only_people_on_query.get_mut(leaved_cell)
            {
                people_on.0 -= 1;
                is_activated.0 = people_on.0 > 0;
                // println!("people on: {}", people_on.0);
            }
        }
        if let Some(entered_cell) = object_map.cells.get(&new_position_event.get_now()) {
            // println!("entered in cell {:?}, at position {:?}", entered_cell, new_position_event.get_now());
            match player_only_people_on_query.get_mut(*entered_cell) {
                Ok((mut is_activated, mut people_on)) => {
                    people_on.0 += 1;
                    is_activated.0 = people_on.0 > 0;
                    // println!("people on: {}", people_on.0);
                }
                Err(_err) => {
                    // println!("{:?}: this cell has no PeopleOn", err);
                }
            }
        }
    }
}
