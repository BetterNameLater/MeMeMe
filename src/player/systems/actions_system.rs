use crate::constantes::*;
use crate::items::events::OnEnterEvent;
use crate::items::events::OnExitEvent;
use crate::items::interaction_type::InteractionType;
use crate::items::primitive::colliding::Colliding;
use crate::items::primitive::enterable::EnterAble;
use crate::items::primitive::interactible::Interactible;
use crate::items::primitive::is_usable::IsUsable;
use crate::level::ressources::level_informations::PlayingTime;
use crate::player::actions::ActionStack;
use crate::player::actions::{Action, ActionType};
use crate::player::components::person::Person;
use crate::player::events::interact_event::InteractEvent;
use crate::player::events::new_position_event::NewPositionEventData;
use bevy::prelude::*;

/// Process the ghost actions
#[allow(clippy::too_many_arguments)]
pub fn actions_system<P: Person, W: InteractionType>(
    mut action_stack: ResMut<ActionStack<P>>,
    playing_time: Res<PlayingTime>,

    mut interact_event: EventWriter<InteractEvent<P>>,

    mut person_transform_query: Query<&mut Transform, With<P>>,
    mut on_enter_event_writer: EventWriter<OnEnterEvent>,
    mut on_exit_event_writer: EventWriter<OnExitEvent>,

    enterables_query: Query<(Entity, &Transform), (With<EnterAble>, Without<W>, Without<P>)>,
    interactibles_query: Query<(Entity, &Transform), (With<Interactible>, Without<W>, Without<P>)>,
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
                    let before: IVec2 = person_transform.translation.xy().as_ivec2();
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
                            now: person_transform.translation.xy().as_ivec2(),
                            entity: *ghost_entity,
                        };
                        add_enter_exit_event(
                            new_position_event,
                            &enterables_query,
                            &mut on_enter_event_writer,
                            &mut on_exit_event_writer,
                        );
                    }
                }
                ActionType::Interact => {
                    interactibles_query
                        .iter()
                        .filter(|(_, t)| person_transform.translation.xy() == t.translation.xy())
                        .for_each(|(item, _)| {
                            debug!("{:?} interacted with {:?}!", ghost_entity, item);
                            interact_event.send(InteractEvent::new(*ghost_entity, item));
                        });
                }
            }
        }
    }
}

fn add_enter_exit_event<P: Person, W: InteractionType>(
    new_position_event: NewPositionEventData,
    player_only_people_on_query: &Query<
        (Entity, &Transform),
        (With<EnterAble>, Without<W>, Without<P>),
    >,
    on_enter_event_writer: &mut EventWriter<OnEnterEvent>,
    on_exit_event_writer: &mut EventWriter<OnExitEvent>,
) {
    player_only_people_on_query
        .iter()
        .filter(|(_, t)| new_position_event.now == t.translation.xy().as_ivec2())
        .for_each(|(entered_cell, _)| {
            debug!(
                "{:?} was entered by {:?}!",
                entered_cell, new_position_event.entity
            );
            on_enter_event_writer.send(OnEnterEvent {
                item: entered_cell,
                person: new_position_event.entity,
            });
        });

    player_only_people_on_query
        .iter()
        .filter(|(_, t)| new_position_event.before == t.translation.xy().as_ivec2())
        .for_each(|(leaved_cell, _)| {
            debug!(
                "{:?} was exit by {:?}!",
                leaved_cell, new_position_event.entity
            );
            on_exit_event_writer.send(OnExitEvent {
                item: leaved_cell,
                person: new_position_event.entity,
            });
        });
}
