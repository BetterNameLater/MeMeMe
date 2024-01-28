use crate::constantes::*;
use crate::level::ressources::level_informations::LevelInformations;
use crate::map::{Map, ObjectMap};
use crate::math::vec2i::Vec2i;
use crate::player::actions::{Action, ActionType};
use crate::player::components::player::Player;
use crate::player::events::interact_event::InteractEvent;
use crate::player::events::new_position_event::NewPositionEvent;
use crate::player::events::rewind_event::RewindEvent;
use crate::player::move_direction::MoveDirection;
use crate::player::PlayerNewPositionEvent;
use bevy::prelude::*;

#[allow(clippy::too_many_arguments)]
pub fn player_input_system(
    mut player_transform_query: Query<(&mut Transform, Entity), With<Player>>,
    mut player_query: Query<&mut Player>,
    key_inputs: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut level_infos: ResMut<LevelInformations>,
    mut rewind_event: EventWriter<RewindEvent>,
    mut player_new_position_event: EventWriter<PlayerNewPositionEvent>,
    mut player_interact_event: EventWriter<InteractEvent<Player>>,
    object_map_query: Query<&Map, With<ObjectMap>>,
) {
    // move actions
    let move_key = key_inputs.get_just_pressed().find(|&&key_code| {
        matches!(
            key_code,
            INPUT_PLAYER_DOWN | INPUT_PLAYER_UP | INPUT_PLAYER_LEFT | INPUT_PLAYER_RIGHT
        )
    });
    let (mut player_transform, player_entity) = player_transform_query.single_mut();

    if let Some(move_key) = move_key {
        let move_direction = MoveDirection::from_key_code(*move_key);
        let before: Vec2i = player_transform.translation.into();
        player_transform.translation += CELL_LENGTH * move_direction.to_vec3();
        player_query.single_mut().actions.push(Action {
            ghost_entity: player_entity,
            action_type: ActionType::Move(move_direction),
            timestamp_seconds: level_infos.elapsed_time_from_start_rewind.unwrap_or(0.),
        });
        if level_infos.elapsed_time_from_start_rewind.is_none() {
            level_infos.start_time = Some(time.elapsed_seconds());
            level_infos.elapsed_time_from_start_rewind = Some(0.);
        }

        /*
        TODO : OnEnterEvent
         */
        player_new_position_event.send(PlayerNewPositionEvent::new(
            before,
            player_transform.translation.into(),
            player_entity,
        ));
        return;
    }

    // other actions
    let action_key = key_inputs
        .get_just_pressed()
        .find(|key_code| matches!(**key_code, INPUT_PLAYER_REWIND | INPUT_PLAYER_INTERACT));
    if let Some(action_key) = action_key {
        match *action_key {
            INPUT_PLAYER_REWIND => {
                rewind_event.send(RewindEvent);
            }
            INPUT_PLAYER_INTERACT => {
                player_query.single_mut().actions.push(Action {
                    ghost_entity: player_entity,
                    action_type: ActionType::Interact,
                    timestamp_seconds: level_infos.elapsed_time_from_start_rewind.unwrap_or(0.),
                });
                let object_map = object_map_query.single();
                let pos: Vec2i = player_transform.translation.into();
                if let Some(item) = object_map.cells.get(&pos) {
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
