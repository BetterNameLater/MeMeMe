use crate::items::components::enterable::EnterAble;
use crate::items::components::player_only::PersonOnly;
use crate::items::events::{OnEnterEvent, OnExitEvent};
use crate::map::{Map, ObjectMap};
use crate::player::events::new_position_event::NewPositionEvent;
use bevy::prelude::*;

pub fn people_enter_system<W: PersonOnly, E: NewPositionEvent>(
    mut player_new_position_event: EventReader<E>,
    player_only_people_on_query: Query<(With<EnterAble>, Without<W>)>,
    object_map_query: Query<&Map, With<ObjectMap>>,
    mut on_enter_event_writer: EventWriter<OnEnterEvent>,
    mut on_exit_event_writer: EventWriter<OnExitEvent>,
) {
    let object_map = object_map_query.single();
    for new_position_event in player_new_position_event.read() {
        if let Some(entered_cell) = object_map.cells.get(&new_position_event.get_now()) {
            if player_only_people_on_query.contains(*entered_cell) {
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

        if let Some(leaved_cell) = object_map.cells.get(&new_position_event.get_before()) {
            if player_only_people_on_query.contains(*leaved_cell) {
                println!(
                    "{:?} was exit by {:?}!",
                    leaved_cell,
                    new_position_event.get_entity()
                );
                on_exit_event_writer.send(OnExitEvent {
                    position: new_position_event.get_now(),
                    item: *leaved_cell,
                    person: new_position_event.get_entity(),
                });
            }
        }
    }
}
