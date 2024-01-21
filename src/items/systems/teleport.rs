use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;

use crate::items::is_usable::IsUsable;
use crate::player::events::NewPositionEvent;
use crate::{
    map::{Map, ObjectMap},
    math::vec2i::Vec2i,
};

#[derive(Component)]
pub struct Teleporter(pub Vec2i);

// teleport entities that enter the teleporter
pub fn teleporter_system<W: Component, E: NewPositionEvent, T: Component>(
    mut player_new_position_event: EventReader<E>,
    object_map_query: Query<&Map, With<ObjectMap>>,
    teleporter_query: Query<&Teleporter, (With<IsUsable>, Without<W>)>,
    mut player_query: Query<&mut Transform, With<T>>,
) {
    // TODO
    if object_map_query.is_empty() {
        return;
    }
    let map = object_map_query.single();
    for event in player_new_position_event.read() {
        let player = player_query.get_mut(event.get_entity());
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

// Might be Refactored
// teleport all entities already on the teleporter if it's activated
pub fn teleporter_activate_system<W: Component, T: Component>(
    teleporter_query: Query<(&Teleporter, &Transform), (Changed<IsUsable>, Without<W>)>,
    mut entities_query: Query<(Entity, &mut Transform), (With<T>, Without<Teleporter>)>,
) {
    let mut moved: HashMap<Entity, ()> = HashMap::new();
    for (dest, src) in teleporter_query.iter() {
        let mut entities: Vec<(Entity, Mut<'_, bevy::prelude::Transform>)> = entities_query
            .iter_mut()
            .filter(|(_, e)| {
                e.translation.x == src.translation.x && e.translation.y == src.translation.y
            })
            .collect();
        for (entity, transform) in entities.iter_mut() {
            if moved.contains_key(entity) {
                continue;
            }
            transform.translation = Vec3::new(dest.0.x as f32, dest.0.y as f32, 1.);
            moved.insert(*entity, ());
        }
    }
}
