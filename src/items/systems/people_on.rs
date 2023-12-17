use crate::items::ghost_only::GhostOnly;
use crate::items::player_only::PlayerOnly;
use crate::map::{Map, ObjectMap};
use crate::math::vec2i::Vec2i;
use crate::player::events::NewPositionEvent;
use crate::player::player::Player;
use crate::player::{Ghost, GhostNewPositionEvent, PlayerNewPositionEvent};
use bevy::prelude::*;
use crate::items::is_usable::IsUsable;

use super::is_activated::{IsActivated, self};

/// The number of [`Ghost`] and [`Player`] on this position
#[derive(Component)]
pub struct PeopleOn(pub usize);

pub fn count_people_on_system<W: Component, E: NewPositionEvent>(
    mut player_new_position_event: EventReader<E>,
    mut player_only_people_on_query: Query<(&mut IsActivated, &mut PeopleOn), (With<IsUsable>, Without<W>)>,
    mut object_map_query: Query<&Map, With<ObjectMap>>,
) {
    // TODO le system ne devait pas ce lancé tant que la map n'est pas lancée
    if object_map_query.is_empty() {
        return;
    }
    let object_map = object_map_query.single();
    for new_position_event in player_new_position_event.read() {
		println!("count_people_on_system");
        if let Some(&leaved_cell) =
            object_map.cells.get(&new_position_event.get_before())
        {
            if let Ok((mut is_activated, mut people_on)) = player_only_people_on_query.get_mut(leaved_cell) {
                people_on.0 -= 1;
				is_activated.0 = people_on.0 > 0;
				println!("people on: {}", people_on.0);
            }
        }
        if let Some(&entered_cell) = object_map.cells.get(&new_position_event.get_now())
        {
			println!("item here");
            if let Ok((mut is_activated, mut people_on)) = player_only_people_on_query.get_mut(entered_cell) {
                people_on.0 += 1;
				is_activated.0 = people_on.0 > 0;
				println!("people on: {}", people_on.0);
            }
        }
    }
}
