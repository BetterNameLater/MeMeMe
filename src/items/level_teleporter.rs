use crate::items::enterable::EnterAble;
use crate::items::events::OnEnterEvent;
use crate::items::is_usable::IsUsable;
use crate::items::player_only::PlayerOnly;
use crate::map::{Map, ObjectMap};
use bevy::prelude::*;

#[derive(Component)]
pub struct LevelTeleporter(pub String);

pub fn level_teleporter_system(
    mut on_enter_event_reader: EventReader<OnEnterEvent>,
    level_teleporter_query: Query<
        &LevelTeleporter,
        (
            With<EnterAble>,
            With<IsUsable>,
            With<PlayerOnly>,
            With<LevelTeleporter>,
        ),
    >,
    object_map_query: Query<&Map, With<ObjectMap>>,
) {
    // TODO le system ne devait pas ce lancé tant que la map n'est pas lancée
    if object_map_query.is_empty() {
        return;
    }
    let _object_map = object_map_query.single();
    for on_enter_event in on_enter_event_reader.read() {
        if let Ok(level_name) = level_teleporter_query.get(on_enter_event.item) {
            println!("goto level {}", level_name.0);
        }
    }
}
