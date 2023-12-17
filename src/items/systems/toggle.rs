use bevy::prelude::*;

use super::is_activated::IsActivated;
use crate::items::is_usable::IsUsable;
use crate::player::events::NewPositionEvent;
use crate::player::interact::InteractEvent;
use crate::{
    map::{Map, ObjectMap},
    math::vec2i::Vec2i,
    player::{self, player::Player, Ghost, GhostNewPositionEvent, PlayerNewPositionEvent},
};

#[derive(Component)]
pub struct ToggleInteract;
#[derive(Component)]
pub struct ToggleOnEnter;

// toggle entitie isActive when player interact with it
pub fn toggle_on_interact_system<W: Component, E: InteractEvent>(
    mut player_new_position_event: EventReader<E>,
    object_map_query: Query<&Map, With<ObjectMap>>,
    mut levers_query: Query<&mut IsActivated, (With<IsUsable>, With<ToggleInteract>, Without<W>)>,
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

// toggle entitie isActive when player enter it
pub fn toggle_on_enter_system<W: Component, E: NewPositionEvent>(
    mut player_new_position_event: EventReader<E>,
    object_map_query: Query<&Map, With<ObjectMap>>,
    mut levers_query: Query<&mut IsActivated, (With<IsUsable>, With<ToggleOnEnter>, Without<W>)>,
) {
    // TODO
    if object_map_query.is_empty() {
        return;
    }
    let map = object_map_query.single();
    for event in player_new_position_event.read() {
		let item_at = map.cells.get(&event.get_now());
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
