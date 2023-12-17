use bevy::prelude::*;

use super::{ghost_only::GhostOnly, people_on::PeopleOn};
use crate::player::events::NewPositionEvent;
use crate::player::interact::InteractEvent;
use crate::{
    map::{Map, ObjectMap},
    math::vec2i::Vec2i,
    player::{self, player::Player, Ghost, GhostNewPositionEvent, PlayerNewPositionEvent},
};

#[derive(Component)]
pub struct Toggle(pub bool);


pub fn interact_toggle_system<W: Component, E: InteractEvent>(
    mut player_new_position_event: EventReader<E>,
    object_map_query: Query<&Map, With<ObjectMap>>,
    mut levers_query: Query<&mut Toggle, Without<W>>,
) {
    // TODO
    if object_map_query.is_empty() {
        return;
    }
    let map = object_map_query.single();
    for event in player_new_position_event.read() {
		let item_at = map.cells.get(&event.get_pos());
        if item_at.is_none() {
            continue;
        }
        let item_at = item_at.unwrap();

        if let Ok(mut toggle) = levers_query.get_mut(*item_at) {
			toggle.0 = !toggle.0;
            println!("lever toggled to {}", toggle.0);
        }
    }
}
