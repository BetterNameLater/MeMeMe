use crate::constantes::*;
use crate::items::events::OnEnterEvent;
use crate::items::events::OnExitEvent;
use crate::items::interaction_type::ghost_only::GhostOnly;
use crate::items::interaction_type::InteractionType;
use crate::items::primitive::colliding::Colliding;
use crate::items::primitive::enterable::EnterAble;
use crate::items::primitive::is_usable::IsUsable;
use crate::level::level_state::LevelState;
use crate::level::ressources::level_informations::LevelInformations;
use crate::map::ObjectMap;
use crate::math::vec2i::Vec2i;
use crate::player::actions::{Action, ActionType};
use crate::player::components::player::Player;
use crate::player::events::interact_event::InteractEvent;
use crate::player::events::new_position_event::NewPositionEventData;
use crate::player::move_direction::MoveDirection;
use bevy::prelude::*;

#[allow(clippy::too_many_arguments)]
pub fn player_move_input_system(
    mut player_transform_query: Query<(&mut Transform, Entity), With<Player>>,
    mut player_query: Query<&mut Player>,
    key_inputs: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut level_infos: ResMut<LevelInformations>,
    object_map_query: Query<&ObjectMap>,
    player_only_people_on_query: Query<(), (With<EnterAble>, Without<GhostOnly>)>,
    mut on_enter_event_writer: EventWriter<OnEnterEvent>,
    mut on_exit_event_writer: EventWriter<OnExitEvent>,
    colliding_query: Query<
        (&Colliding, &Transform, Option<&IsUsable>),
        (Without<GhostOnly>, Without<Player>),
    >,
) {
    // move actions
    let move_key = key_inputs.get_just_pressed().find(|&&key_code| {
        matches!(
            key_code,
            INPUT_PLAYER_DOWN | INPUT_PLAYER_UP | INPUT_PLAYER_LEFT | INPUT_PLAYER_RIGHT
        )
    });

    if let Some(move_key) = move_key {
        let (mut player_transform, player_entity) = player_transform_query.single_mut();
        let move_direction = MoveDirection::from_key_code(*move_key);
        let before: Vec2i = player_transform.translation.into();
        player_query.single_mut().actions.push(Action {
            ghost_entity: player_entity,
            action_type: ActionType::Move(move_direction),
            timestamp_seconds: level_infos.elapsed_time_from_start_rewind.unwrap_or(0.),
        });
        if level_infos.elapsed_time_from_start_rewind.is_none() {
            level_infos.start_time = Some(time.elapsed_secs());
            level_infos.elapsed_time_from_start_rewind = Some(0.);
        }

        let new_position = player_transform.translation + CELL_LENGTH * move_direction.to_vec3();
        // TODO: should we record the action, even if the player collides ?
        let collide = colliding_query
            .iter()
            .any(|(_, colliding_transform, is_usable)| {
                colliding_transform.translation.x == new_position.x
                    && colliding_transform.translation.y == new_position.y
                    && is_usable.is_none()
            });
        if !collide {
            player_transform.translation = new_position;
            let new_position_event = NewPositionEventData {
                before,
                now: player_transform.translation.into(),
                entity: player_entity,
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
}

pub fn add_enter_exit_event<W: InteractionType>(
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

pub fn player_action_input_system(
    mut player_transform_query: Query<(&mut Transform, Entity), With<Player>>,
    mut player_query: Query<&mut Player>,
    key_inputs: Res<ButtonInput<KeyCode>>,
    level_infos: ResMut<LevelInformations>,
    mut player_interact_event: EventWriter<InteractEvent<Player>>,
    object_map_query: Query<&ObjectMap>,
    mut next_state: ResMut<NextState<LevelState>>,
) {
    let action_key = key_inputs
        .get_just_pressed()
        .find(|key_code| matches!(**key_code, INPUT_PLAYER_REWIND | INPUT_PLAYER_INTERACT));
    if let Some(action_key) = action_key {
        let (player_transform, player_entity) = player_transform_query.single_mut();
        match *action_key {
            INPUT_PLAYER_REWIND => {
                if level_infos.elapsed_time_from_start_rewind.is_none() {
                    debug!("Rewind without actual start");
                } else {
                    next_state.set(LevelState::Rewind);
                }
            }
            INPUT_PLAYER_INTERACT => {
                player_query.single_mut().actions.push(Action {
                    ghost_entity: player_entity,
                    action_type: ActionType::Interact,
                    timestamp_seconds: level_infos.elapsed_time_from_start_rewind.unwrap_or(0.),
                });
                let object_map = object_map_query.single();
                let pos: Vec2i = player_transform.translation.into();
                if let Some(item) = object_map.0.get(&pos) {
                    player_interact_event.send(InteractEvent::new(
                        player_transform.translation.into(),
                        player_entity,
                        *item,
                    ));
                }
            }
            _ => unreachable!(),
        }
    }
}
