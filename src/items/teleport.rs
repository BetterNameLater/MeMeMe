use bevy::prelude::*;

use super::{ghost_only::GhostOnly, people_on::PeopleOn};
use crate::player::events::NewPositionEvent;
use crate::{
    map::{Map, ObjectMap},
    math::vec2i::Vec2i,
    player::{self, player::Player, Ghost, GhostNewPositionEvent, PlayerNewPositionEvent},
};

#[derive(Component)]
pub struct Teleporter(pub Vec2i);

pub fn teleporter_system<W: Component, E: NewPositionEvent, T: Component>(
    mut player_new_position_event: EventReader<E>,
    object_map_query: Query<&Map, With<ObjectMap>>,
    teleporter_query: Query<&Teleporter, Without<W>>,
    mut player_query: Query<&mut Transform, With<T>>,
) {
    // TODO
    if object_map_query.is_empty() {
        return;
    }
    let map = object_map_query.single();
    for event in player_new_position_event.read() {
        let mut player = player_query.get_mut(event.get_entity());
        if player.is_err() {
            continue;
        }
        let mut player = player.unwrap();
        if map.cells.get(&event.get_now()).is_none() {
            continue;
        }
        let pos = map.cells.get(&event.get_now()).unwrap();
        if let Ok(destination) = teleporter_query.get(*pos) {
            player.translation = Vec3::new(destination.0.x as f32, destination.0.y as f32, 1.);
            println!("teleporting player");
        }
    }
}
