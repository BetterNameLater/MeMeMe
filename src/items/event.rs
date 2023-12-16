use crate::items::pressure_plate::PressurePlate;
use crate::math::vec2i::Vec2i;
use crate::player::{OnEnterEvent, RewindEvent};
use bevy::prelude::{EventReader, Query};
use bevy::{prelude::*, transform};



pub fn on_enter_system(
    mut rewind_event: EventReader<OnEnterEvent>,
	// mut enterable_items: Query<(&mut Enterable, &Transform)>,
	mut pressure_plate_query: Query<(&mut PressurePlate, &Transform)>,
) {
    for event in rewind_event.read() {

        println!("event {:?}", event.0);

		// PressurePlate
        for (mut pressure_plate, transform) in pressure_plate_query.iter_mut() {
           if Vec2i::from_vec3(transform.translation) == event.0 {
		   		pressure_plate.level += 1;
				print!("on enter {:?}\n", pressure_plate.level);
				break;
		   }
        }

    }
}

pub fn on_exit_system(
    mut rewind_event: EventReader<OnEnterEvent>,
	mut pressure_plate_query: Query<(&mut PressurePlate, &Transform)>,
) {
    for event in rewind_event.read() {


        println!("event {:?}", event.0);

		// PressurePlate
        for (mut pressure_plate, transform) in pressure_plate_query.iter_mut() {
           if Vec2i::from_vec3(transform.translation) == event.0 {
		   		pressure_plate.level += 1;
				print!("on enter {:?}\n", pressure_plate.level);
				break;
		   }
        }

    }
}
