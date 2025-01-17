use crate::constantes::*;
use crate::items::events::OnEnterEvent;
use crate::items::events::OnExitEvent;
use crate::items::interaction_type::player_only::PlayerOnly;
use crate::items::primitive::colliding::Colliding;
use crate::items::primitive::enterable::EnterAble;
use crate::items::primitive::is_usable::IsUsable;
use crate::level::ressources::level_informations::PlayingTime;
use crate::map::ObjectMap;
use crate::player::actions::ActionStack;
use crate::player::actions::{Action, ActionType};
use crate::player::events::interact_event::InteractEvent;
use crate::player::events::new_position_event::NewPositionEventData;
use crate::player::{systems::player_input_system::add_enter_exit_event, Ghost};
use bevy::prelude::*;
use maths::Vec2i;

/// Process the ghost actions
#[allow(clippy::too_many_arguments)]
pub fn ghost_actions_system(
    mut ghost_actions: ResMut<ActionStack<Ghost>>,
    mut ghosts_query: Query<&mut Transform, With<Ghost>>,
    playing_time: Res<PlayingTime>,
    mut ghost_interact_event: EventWriter<InteractEvent<Ghost>>,
    object_map_query: Query<&ObjectMap>,
    player_only_people_on_query: Query<(), (With<EnterAble>, Without<PlayerOnly>)>,
    mut on_enter_event_writer: EventWriter<OnEnterEvent>,
    mut on_exit_event_writer: EventWriter<OnExitEvent>,
    colliding_query: Query<
        (&Colliding, &Transform, Option<&IsUsable>),
        (Without<PlayerOnly>, Without<Ghost>),
    >,
) {
    while let Some(actions) = ghost_actions.exec(playing_time.0.elapsed()) {
        for action in actions.iter() {
            let Action {
                ghost_entity,
                action_type,
            } = action;

            let mut ghost_transform = ghosts_query.get_mut(*ghost_entity).unwrap();
            match action_type {
                ActionType::Move(move_direction) => {
                    let before: Vec2i = ghost_transform.translation.into();
                    let new_position =
                        ghost_transform.translation + CELL_LENGTH * move_direction.to_vec3();

                    let collide =
                        colliding_query
                            .iter()
                            .any(|(_, colliding_transform, is_usable)| {
                                colliding_transform.translation.x == new_position.x
                                    && colliding_transform.translation.y == new_position.y
                                    && is_usable.is_none()
                            });
                    if !collide {
                        ghost_transform.translation = new_position;
                        let new_position_event = NewPositionEventData {
                            before,
                            now: ghost_transform.translation.into(),
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
                    let pos: Vec2i = ghost_transform.translation.into();
                    if let Some(item) = object_map.0.get(&pos) {
                        ghost_interact_event.send(InteractEvent::new(
                            ghost_transform.translation.into(),
                            *ghost_entity,
                            *item,
                        ));
                    }
                }
            }
        }
    }
}
