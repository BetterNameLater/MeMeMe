use crate::items::primitive::is_activated::IsActivated;
use crate::items::primitive::is_usable::IsUsable;
use crate::items::primitive::people_on::PeopleOn;
use crate::items::primitive::player_only::PersonOnly;
use crate::items::events::{OnEnterEvent, OnExitEvent};
use crate::player::components::player::Person;
use bevy::prelude::*;

pub fn count_people_on_system<W: PersonOnly, T: Person>(
    mut player_only_people_on_query: Query<
        (&mut IsActivated, &mut PeopleOn),
        (With<IsUsable>, Without<W>),
    >,
    person: Query<(), With<T>>,
    mut on_exit_event_reader: EventReader<OnExitEvent>,
    mut on_enter_event_reader: EventReader<OnEnterEvent>,
) {
    on_exit_event_reader
        .read()
        .filter(|on_exit_event| person.contains(on_exit_event.person))
        .for_each(|on_exit_event| {
            if let Ok((mut is_activated, mut people_on)) =
                player_only_people_on_query.get_mut(on_exit_event.item)
            {
                people_on.0 -= 1;
                is_activated.0 = people_on.0 > 0;
                debug!("people on (on exit): {}", people_on.0);
            }
        });

    on_enter_event_reader
        .read()
        .filter(|on_enter_event| person.contains(on_enter_event.person))
        .for_each(|on_enter_event| {
            if let Ok((mut is_activated, mut people_on)) =
                player_only_people_on_query.get_mut(on_enter_event.item)
            {
                people_on.0 += 1;
                is_activated.0 = people_on.0 > 0;
                debug!("people on (on enter): {}", people_on.0);
            }
        });
}
