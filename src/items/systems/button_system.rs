use crate::items::interaction_type::InteractionType;
use crate::items::primitive::cooldown::Cooldown;
use crate::items::primitive::is_activated::IsActivated;
use crate::items::primitive::is_usable::IsUsable;
use crate::items::primitive::pressable::Pressable;
use crate::player::components::person::Person;
use crate::player::events::interact_event::InteractEvent;
use bevy::prelude::*;

pub fn button_pressed_system<W: InteractionType, T: Person>(
    mut commands: Commands,
    mut cooldown_query: Query<
        (Option<&mut Cooldown>, &mut IsActivated),
        (Without<W>, With<Pressable>, With<IsUsable>),
    >,
    person: Query<(), With<T>>,
    mut on_interact_event_reader: EventReader<InteractEvent<T>>,
) {
    for on_interact_event in on_interact_event_reader.read() {
        if !person.contains(on_interact_event.person) {
            continue;
        }
        if let Ok(query) = cooldown_query.get_mut(on_interact_event.item) {
            let (some_timer, mut is_activated) = query;
            if let Some(mut timer) = some_timer {
                debug!("Pressing a button already cooling down");
                timer.0.reset();
            } else {
                is_activated.0 = true;
                commands
                    .entity(on_interact_event.item)
                    .insert(Cooldown::new());
                debug!("Pressing a button and adding a Cooldown");
            }
        }
    }
}
