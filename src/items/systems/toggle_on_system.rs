use crate::items::components::is_activated::IsActivated;
use crate::items::components::is_usable::IsUsable;
use crate::items::components::player_only::PersonOnly;
use crate::items::components::toggle::{Enter, Interact, Toggle};
use crate::items::events::OnEnterEvent;
use crate::player::components::player::Person;
use crate::player::events::interact_event::InteractEvent;
use bevy::prelude::*;

// toggle entitie isActive when player enter it
pub fn toggle_on_enter_system<W: PersonOnly, T: Person>(
    mut toggle_query: Query<
        &mut IsActivated,
        (
            With<IsUsable>,
            With<Toggle<Enter>>,
            Without<W>,
            With<IsUsable>,
        ),
    >,
    person: Query<(), With<T>>,
    mut on_enter_event_reader: EventReader<OnEnterEvent>,
) {
    for on_enter_event in on_enter_event_reader.read() {
        if !person.contains(on_enter_event.person) {
            continue;
        }
        if let Ok(mut toggle) = toggle_query.get_mut(on_enter_event.item) {
            toggle.0 = !toggle.0;
            debug!("lever toggled to {}", toggle.0);
        }
    }
}

pub fn toggle_on_interact_system<W: PersonOnly, T: Person>(
    mut toggle_query: Query<
        &mut IsActivated,
        (
            With<IsUsable>,
            With<Toggle<Interact>>,
            Without<W>,
            With<IsUsable>,
        ),
    >,
    person: Query<(), With<T>>,
    mut on_interact_event_reader: EventReader<InteractEvent<T>>,
) {
    for on_interact_event in on_interact_event_reader.read() {
        if !person.contains(on_interact_event.person) {
            continue;
        }
        if let Ok(mut toggle) = toggle_query.get_mut(on_interact_event.item) {
            toggle.0 = !toggle.0;
            debug!("lever toggled to {}", toggle.0);
        }
    }
}
