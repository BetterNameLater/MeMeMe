use crate::constantes::*;
use crate::items::events::OnEnterEvent;
use crate::items::events::OnExitEvent;
use crate::items::interaction_type::InteractionType;
use crate::items::primitive::colliding::Colliding;
use crate::items::primitive::enterable::EnterAble;
use crate::items::primitive::is_usable::IsUsable;
use crate::level::ressources::level_informations::PlayingTime;
use crate::player::actions::ActionStack;
use crate::player::actions::{Action, ActionType};
use crate::player::components::person::Person;
use crate::player::events::new_position_event::NewPositionEventData;
use bevy::prelude::*;

/// Process the ghost actions
#[allow(clippy::too_many_arguments)]
pub fn actions_system<P: Person, W: InteractionType>(
    mut action_stack: ResMut<ActionStack<P>>,
    playing_time: Res<PlayingTime>,

    mut on_enter_event_writer: EventWriter<OnEnterEvent>,
    mut on_exit_event_writer: EventWriter<OnExitEvent>,

    enterables_query: Query<(Entity, &Transform), (With<EnterAble>, Without<W>, Without<P>)>,
    mut transforms: ParamSet<(
        Query<&mut Transform, With<P>>,
        Query<(&Colliding, &Transform, Option<&IsUsable>), Without<W>>,
    )>,
) {
    while let Some(actions) = action_stack.exec(playing_time.0.elapsed()) {
        for action in actions.iter() {
            let Action {
                ghost_entity,
                action_type,
            } = action;

            let person_transform = transforms.p0().get(*ghost_entity).unwrap().translation;

            match action_type {
                ActionType::Move(move_direction) => {
                    let before: IVec2 = transforms
                        .p0()
                        .get(*ghost_entity)
                        .unwrap()
                        .translation
                        .xy()
                        .as_ivec2();
                    let Some(blocking) =
                        raycast_2d(person_transform, move_direction.to_vec3(), &transforms.p1())
                    else {
                        unimplemented!("go into void");
                    };
                    let new_position = blocking - CELL_LENGTH * move_direction.to_vec3();

                    transforms.p0().get_mut(*ghost_entity).unwrap().translation = new_position;

                    let new_position_event = NewPositionEventData {
                        before,
                        now: new_position.xy().as_ivec2(),
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
        }
    }
}

const LIMIT: usize = 24;
fn raycast_2d<W: InteractionType>(
    base_position: Vec3,
    move_direction: Vec3,
    colliding_query: &Query<(&Colliding, &Transform, Option<&IsUsable>), Without<W>>,
) -> Option<Vec3> {
    (1..LIMIT).find_map(|i| {
        let new_position = base_position + (CELL_LENGTH * move_direction * i as f32);
        if colliding_query
            .iter()
            .any(|(_, colliding_transform, is_usable)| {
                colliding_transform.translation.x == new_position.x
                    && colliding_transform.translation.y == new_position.y
                    && is_usable.is_none() // door not usable, so blocking
            })
        {
            Some(new_position)
        } else {
            None
        }
    })
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
