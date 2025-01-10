use crate::items::components::cooldown::Cooldown;
use crate::items::components::is_activated::IsActivated;
use crate::items::components::is_usable::IsUsable;
use crate::items::components::player_only::PersonOnly;
use crate::items::components::pressable::Pressable;
use crate::player::components::player::Person;
use crate::player::events::interact_event::InteractEvent;
use bevy::prelude::*;

pub fn button_pressed_system<W: PersonOnly, T: Person>(
    mut commands: Commands,
    mut cooldown_query: Query<
        (Option<&mut Cooldown<IsActivated>>, &mut IsActivated),
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
                    .insert(Cooldown::<IsActivated>::new(1.));
                debug!("Pressing a button and adding a Cooldown");
            }
        }
    }
}
