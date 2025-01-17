use crate::constantes::*;
use crate::items::events::OnEnterEvent;
use crate::items::events::OnExitEvent;
use crate::items::interaction_type::InteractionType;
use crate::items::primitive::colliding::Colliding;
use crate::items::primitive::enterable::EnterAble;
use crate::items::primitive::is_usable::IsUsable;
use crate::level::ressources::level_informations::PlayingTime;
use crate::map::ObjectMap;
use crate::player::actions::ActionStack;
use crate::player::actions::{Action, ActionType};
use crate::player::components::person::Person;
use crate::player::events::interact_event::InteractEvent;
use crate::player::events::new_position_event::NewPositionEventData;
use bevy::prelude::*;
use maths::Vec2i;

/// Process the ghost actions
#[allow(clippy::too_many_arguments)]
pub fn actions_system<P: Person, W: InteractionType>(
    mut action_stack: ResMut<ActionStack<P>>,
    playing_time: Res<PlayingTime>,

    mut interact_event: EventWriter<InteractEvent<P>>,

    mut person_transform_query: Query<&mut Transform, With<P>>,
    mut on_enter_event_writer: EventWriter<OnEnterEvent>,
    mut on_exit_event_writer: EventWriter<OnExitEvent>,

    object_map_query: Query<&ObjectMap>,

    player_only_people_on_query: Query<(), (With<EnterAble>, Without<W>)>,
    colliding_query: Query<(&Colliding, &Transform, Option<&IsUsable>), (Without<W>, Without<P>)>,
) {
    while let Some(actions) = action_stack.exec(playing_time.0.elapsed()) {
        for action in actions.iter() {
            let Action {
                ghost_entity,
                action_type,
            } = action;

            let mut person_transform = person_transform_query.get_mut(*ghost_entity).unwrap();

            match action_type {
                ActionType::Move(move_direction) => {
                    let before: Vec2i = person_transform.translation.into();
                    let new_position =
                        person_transform.translation + CELL_LENGTH * move_direction.to_vec3();

                    let collide =
                        colliding_query
                            .iter()
                            .any(|(_, colliding_transform, is_usable)| {
                                colliding_transform.translation.x == new_position.x
                                    && colliding_transform.translation.y == new_position.y
                                    && is_usable.is_none()
                            });
                    if !collide {
                        person_transform.translation = new_position;
                        let new_position_event = NewPositionEventData {
                            before,
                            now: person_transform.translation.into(),
                            entity: *ghost_entity,
                        };
                        add_enter_exit_event(
                            new_position_event,
                            &object_map_query,
                            &player_only_people_on_query,
                            &mut on_enter_event_writer,
                            &mut on_exit_event_writer,
                        );
                    }
                }
                ActionType::Interact => {
                    let object_map = object_map_query.single();
                    let pos: Vec2i = person_transform.translation.into();
                    if let Some(item) = object_map.0.get(&pos) {
                        interact_event.send(InteractEvent::new(
                            person_transform.translation.into(),
                            *ghost_entity,
                            *item,
                        ));
                    }
                }
            }
        }
    }
}

fn add_enter_exit_event<W: InteractionType>(
    new_position_event: NewPositionEventData,
    object_map_query: &Query<&ObjectMap>,
    player_only_people_on_query: &Query<(), (With<EnterAble>, Without<W>)>,
    on_enter_event_writer: &mut EventWriter<OnEnterEvent>,
    on_exit_event_writer: &mut EventWriter<OnExitEvent>,
) {
    let object_map = object_map_query.single();
    if let Some(entered_cell) = object_map.0.get(&new_position_event.now) {
        if player_only_people_on_query.contains(*entered_cell) {
            debug!(
                "{:?} was entered by {:?}!",
                entered_cell, new_position_event.entity
            );
            on_enter_event_writer.send(OnEnterEvent {
                _position: new_position_event.now,
                item: *entered_cell,
                person: new_position_event.entity,
            });
        }
    }

    if let Some(leaved_cell) = object_map.0.get(&new_position_event.before) {
        if player_only_people_on_query.contains(*leaved_cell) {
            debug!(
                "{:?} was exit by {:?}!",
                leaved_cell, new_position_event.entity
            );
            on_exit_event_writer.send(OnExitEvent {
                _position: new_position_event.now,
                item: *leaved_cell,
                person: new_position_event.entity,
            });
        }
    }
}
