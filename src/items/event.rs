use crate::items::pressure_plate::PressurePlate;
use crate::player::{OnEnterEvent, RewindEvent};
use bevy::prelude::{EventReader, Query};

pub fn on_enter_system(
    mut rewind_event: EventReader<OnEnterEvent>,
    pressure_plate_query: Query<&PressurePlate>,
) {
    for event in rewind_event.read() {
        println!("event {:?}", event.0);
        for wesh in pressure_plate_query.iter() {
            println!("level {:?}", wesh.level);
        }
    }
}
