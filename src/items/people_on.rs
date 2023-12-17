use crate::items::ghost_only::GhostOnly;
use crate::items::player_only::PlayerOnly;
use crate::math::vec2i::Vec2i;
use crate::player::events::NewPositionEvent;
use crate::player::player::Player;
use crate::player::{Ghost, GhostNewPositionEvent, PlayerNewPositionEvent};
use bevy::prelude::*;

/// The number of [`Ghost`] and [`Player`] on this position
#[derive(Component)]
pub struct PeopleOn(pub usize);

pub fn count_people_on_system(
    mut player_new_position_event: EventReader<PlayerNewPositionEvent>,
    mut ghost_new_position_event: EventReader<GhostNewPositionEvent>,
    mut people_on_query: Query<
        (&mut PeopleOn, &Transform),
        (Without<PlayerOnly>, Without<GhostOnly>),
    >,
) {
    let new_position_events = merge_new_positions_event(
        &mut Some(player_new_position_event),
        &mut Some(ghost_new_position_event),
    );
    if new_position_events.is_empty() {
        return;
    }
    for (mut people_on, transform) in &mut people_on_query {
        people_on.0 = count_people_on(
            people_on.0,
            &new_position_events,
            transform.translation.into(),
        );
        println!("new count : {:?}", people_on.0);
    }
}

pub fn count_people_on_ghost_only_system(
    mut ghost_new_position_event: EventReader<GhostNewPositionEvent>,
    mut people_on_query: Query<(&mut PeopleOn, &Transform), (With<GhostOnly>, Without<Player>)>,
) {
    let new_position_events =
        merge_new_positions_event(&mut None, &mut Some(ghost_new_position_event));
    if new_position_events.is_empty() {
        return;
    }
    for (mut people_on, transform) in &mut people_on_query {
        people_on.0 = count_people_on(
            people_on.0,
            &new_position_events,
            transform.translation.into(),
        );
        println!("new count : {:?}", people_on.0);
    }
}

pub fn count_people_on_player_only_system(
    mut player_new_position_event: EventReader<PlayerNewPositionEvent>,
    mut people_on_query: Query<(&mut PeopleOn, &Transform), (With<PlayerOnly>, Without<Ghost>)>,
) {
    let new_position_events =
        merge_new_positions_event(&mut Some(player_new_position_event), &mut None);
    if new_position_events.is_empty() {
        return;
    }
    for (mut people_on, transform) in &mut people_on_query {
        people_on.0 = count_people_on(
            people_on.0,
            &new_position_events,
            transform.translation.into(),
        );
        println!("new count (player only) : {:?}", people_on.0);
    }
}

fn count_people_on(
    original_people_on: usize,
    new_position_events: &Vec<NewPositionEvent>,
    item_position: Vec2i,
) -> usize {
    let mut new_people_on = original_people_on;
    for new_pos in new_position_events {
        if new_pos.now == item_position {
            new_people_on += 1;
        }
        if new_pos.before == item_position {
            new_people_on -= 1;
        }
    }
    new_people_on
}

fn merge_new_positions_event(
    mut player_new_position_event: &mut Option<EventReader<PlayerNewPositionEvent>>,
    mut ghost_new_position_event: &mut Option<EventReader<GhostNewPositionEvent>>,
) -> Vec<NewPositionEvent> {
    let mut p = Vec::new();
    if let Some(player_new_position_event) = player_new_position_event {
        p = player_new_position_event
            .read()
            .map(|ev| NewPositionEvent {
                before: ev.before,
                now: ev.now,
            })
            .collect();
    }
    let mut g = Vec::new();
    if let Some(ghost_new_position_event) = ghost_new_position_event {
        g = ghost_new_position_event
            .read()
            .map(|ev| NewPositionEvent {
                before: ev.before,
                now: ev.now,
            })
            .collect();
    }

    p.append(&mut g);
    p
}
