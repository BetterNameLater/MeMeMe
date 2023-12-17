use crate::items::ghost_only::GhostOnly;
use crate::items::player_only::PlayerOnly;
use crate::map::{Map, ObjectMap};
use crate::math::vec2i::Vec2i;
use crate::player::events::NewPositionEvent;
use crate::player::player::Player;
use crate::player::{Ghost, GhostNewPositionEvent, PlayerNewPositionEvent};
use bevy::prelude::*;

/// The number of [`Ghost`] and [`Player`] on this position
#[derive(Component)]
pub struct PeopleOn(pub usize);

pub fn count_people_on_system<W: Component, E: NewPositionEvent>(
    mut player_new_position_event: EventReader<E>,
    mut player_only_people_on_query: Query<&mut PeopleOn, Without<W>>,
    mut object_map_query: Query<&Map, With<ObjectMap>>,
) {
    // TODO le system ne devait pas ce lancé tant que la map n'est pas lancée
    if object_map_query.is_empty() {
        return;
    }
    let object_map = object_map_query.single();
    for new_position_event in player_new_position_event.read() {
        if let Some(leaved_cell) =
            object_map.get_cell_entity_by_pos(&new_position_event.get_before())
        {
            if let Ok(mut people_on) = player_only_people_on_query.get_mut(leaved_cell) {
                people_on.0 -= 1;
            }
        }
        if let Some(entered_cell) = object_map.get_cell_entity_by_pos(&new_position_event.get_now())
        {
            if let Ok(mut people_on) = player_only_people_on_query.get_mut(entered_cell) {
                people_on.0 += 1;
            }
        }
    }
}
