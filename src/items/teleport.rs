use bevy::prelude::*;

use crate::{player::{PlayerNewPositionEvent, GhostNewPositionEvent, player::Player, Ghost, self}, map::{ObjectMap, Map}, math::vec2i::Vec2i};

use super::{people_on::PeopleOn, ghost_only::GhostOnly};

#[derive(Component)]
pub struct Teleporter(pub Vec2i);

pub fn teleporter_player(
    mut player_new_position_event: EventReader<PlayerNewPositionEvent>,
	object_map_query: Query<&Map, With<ObjectMap>>,
	teleporter_query: Query<&Teleporter, Without<GhostOnly>>,
	mut player_query: Query<&mut Transform, With<Player>>,
) {
	if object_map_query.is_empty() || teleporter_query.is_empty() || player_query.is_empty() {
		return;
	}
    let map = object_map_query.single();
	let mut player = player_query.single_mut();
	for event in player_new_position_event.read(){
		if map.cells.get(&event.now).is_none() {
			return;
		}
		let pos = map.cells.get(&event.now).unwrap();
		if let Ok(destination) = teleporter_query.get(*pos) {
			player.translation = Vec3::new(destination.0.x as f32, destination.0.y as f32, 1.);
			println!("teleporting player");
		}
	}
}


    // mut ghost_new_position_event: EventReader<GhostNewPositionEvent>,