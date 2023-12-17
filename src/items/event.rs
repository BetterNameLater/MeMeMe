use crate::math::vec2i::Vec2i;
use crate::player::{OnEnterEvent, RewindEvent};
use bevy::prelude::{EventReader, Query};
use bevy::{prelude::*, transform};

use super::{Enterable, Population};

fn on_enter_lever(
	mut entity: Entity,
	mut pressure_plate_query: &Query<(&mut Population)>,)
{
	// PressurePlate
	if let Ok(mut pressure_plate) = pressure_plate_query.get_mut(entity) {
		pressure_plate.0 += 1;
	}

}

pub fn on_enter_system(
    mut event: EventReader<OnEnterEvent>,
	mut enterable_items: Query<(Entity, &Transform), With<Enterable>>,
	mut pressure_plate_query: Query<&mut Population>,
) {
    for event in event.read() {
		// get the item
		let mut item: Option<Entity> = None;
        for (entity, transform) in enterable_items.iter_mut() {
           if Vec2i::from_vec3(transform.translation) == event.0 {
			   item = Some(entity);
			   break;
		   }
        }
		if item.is_none() {
			continue;
		}

		let item: Entity = item.unwrap();

		// PressurePlate
		on_enter_lever(item, &pressure_plate_query);

	}
}

// pub fn on_exit_system(
//     mut rewind_event: EventReader<OnEnterEvent>,
// 	mut pressure_plate_query: Query<(&mut PressurePlate, &Transform)>,
// ) {
//     for event in rewind_event.read() {


//         println!("event {:?}", event.0);

// 		// PressurePlate
//         for (mut pressure_plate, transform) in pressure_plate_query.iter_mut() {
//            if Vec2i::from_vec3(transform.translation) == event.0 {
// 		   		pressure_plate.level += 1;
// 				print!("on enter {:?}\n", pressure_plate.level);
// 				break;
// 		   }
//         }

//     }
// }
