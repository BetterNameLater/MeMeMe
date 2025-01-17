use crate::constantes::*;
use crate::level::level_state::LevelState;
use crate::level::ressources::level_informations::PlayingTime;
use crate::map::ObjectMap;
use crate::player::actions::ActionStack;
use crate::player::actions::{Action, ActionType};
use crate::player::components::player::Player;
use crate::player::events::interact_event::InteractEvent;
use crate::player::move_direction::MoveDirection;
use bevy::prelude::*;
use maths::Vec2i;
use std::time::Duration;

#[allow(clippy::too_many_arguments)]
pub fn player_move_input_system(
    mut player_transform_query: Query<Entity, With<Player>>,
    key_inputs: Res<ButtonInput<KeyCode>>,
    level_infos: Option<ResMut<PlayingTime>>, // Ideally, this not be optional -> system that run in Idle just to launch
    current_state: Res<State<LevelState>>,
    mut next_state: ResMut<NextState<LevelState>>,
    mut player_action_stack: ResMut<ActionStack<Player>>,
) {
    let move_key = key_inputs.get_just_pressed().find(|&&key_code| {
        matches!(
            key_code,
            input::DOWN | input::UP | input::LEFT | input::RIGHT
        )
    });

    if let Some(move_key) = move_key {
        let player_entity = player_transform_query.single_mut();
        let move_direction = MoveDirection::from_key_code(*move_key);
        player_action_stack.insert_new(
            Action {
                ghost_entity: player_entity,
                action_type: ActionType::Move(move_direction),
            },
            level_infos.map_or_else(|| Duration::ZERO, |i| i.0.elapsed()),
        );
        if current_state.get() == &LevelState::Idle {
            next_state.set(LevelState::Playing);
        }
    }
}

pub fn player_action_input_system(
    mut player_transform_query: Query<(&mut Transform, Entity), With<Player>>,
    key_inputs: Res<ButtonInput<KeyCode>>,
    level_infos: ResMut<PlayingTime>,
    mut player_interact_event: EventWriter<InteractEvent<Player>>,
    object_map_query: Query<&ObjectMap>,
    mut next_state: ResMut<NextState<LevelState>>,
    current_state: Res<State<LevelState>>,
    mut player_action_stack: ResMut<ActionStack<Player>>,
) {
    let action_key = key_inputs
        .get_just_pressed()
        .find(|key_code| matches!(**key_code, input::REWIND | input::INTERACT));
    if let Some(action_key) = action_key {
        let (player_transform, player_entity) = player_transform_query.single_mut();
        match *action_key {
            input::REWIND => {
                if current_state.get() == &LevelState::Idle {
                    debug!("Trying rewind in Idle State");
                } else {
                    next_state.set(LevelState::Rewind);
                }
            }
            input::INTERACT => {
                player_action_stack.insert_new(
                    Action {
                        ghost_entity: player_entity,
                        action_type: ActionType::Interact,
                    },
                    level_infos.0.elapsed(),
                );

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
