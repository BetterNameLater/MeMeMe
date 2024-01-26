use crate::items::components::enterable::EnterAble;
use crate::items::events::OnEnterEvent;
use crate::map::{Map, ObjectMap};
use crate::player::events::NewPositionEvent;
use bevy::prelude::{Component, EventReader, EventWriter, Query, With, Without};

pub fn people_enter_system<W: Component, E: NewPositionEvent>(
    mut player_new_position_event: EventReader<E>,
    player_only_people_on_query: Query<(With<EnterAble>, Without<W>)>,
    object_map_query: Query<&Map, With<ObjectMap>>,

    mut on_enter_event_writer: EventWriter<OnEnterEvent>,
) {
    let object_map = object_map_query.single();
    for new_position_event in player_new_position_event.read() {
        if let Some(entered_cell) = object_map.cells.get(&new_position_event.get_now()) {
            if player_only_people_on_query.get(*entered_cell).is_ok() {
                println!(
                    "{:?} was entered by {:?}!",
                    entered_cell,
                    new_position_event.get_entity()
                );
                on_enter_event_writer.send(OnEnterEvent {
                    position: new_position_event.get_now(),
                    item: *entered_cell,
                    person: new_position_event.get_entity(),
                });
            }
        }
    }
}
