use super::super::actions::{Action, ActionStack, ActionType};
use crate::constantes::*;
use crate::level::level_state::LevelState;
use crate::level::ressources::level_informations::PlayingTime;
use crate::player::components::player::Player;
use crate::player::move_direction::MoveDirection;
use bevy::prelude::*;
use std::time::Duration;

pub fn is_move_input_pressed(key_inputs: Res<ButtonInput<KeyCode>>) -> bool {
    key_inputs.get_just_pressed().any(|&key_code| {
        matches!(
            key_code,
            input::DOWN | input::UP | input::LEFT | input::RIGHT
        )
    })
}

pub fn idle_move_input_system(
    key_inputs: Res<ButtonInput<KeyCode>>,
    player_query: Query<Entity, With<Player>>,
    mut player_action_stack: ResMut<ActionStack<Player>>,
    mut next_state: ResMut<NextState<LevelState>>,
) {
    let player_entity = player_query.single();
    let move_direction = MoveDirection::from_key_code(get_move_key(key_inputs));

    player_action_stack.insert_new(
        Action {
            ghost_entity: player_entity,
            action_type: ActionType::Move(move_direction),
        },
        Duration::ZERO,
    );
    next_state.set(LevelState::Playing);
}

pub fn move_input_system(
    key_inputs: Res<ButtonInput<KeyCode>>,
    player_query: Query<Entity, With<Player>>,
    playing_time: Res<PlayingTime>,
    mut player_action_stack: ResMut<ActionStack<Player>>,
) {
    let player_entity = player_query.single();
    let move_direction = MoveDirection::from_key_code(get_move_key(key_inputs));

    player_action_stack.insert_new(
        Action {
            ghost_entity: player_entity,
            action_type: ActionType::Move(move_direction),
        },
        playing_time.0.elapsed(),
    );
}

fn get_move_key(key_inputs: Res<ButtonInput<KeyCode>>) -> KeyCode {
    *key_inputs
        .get_just_pressed()
        .find(|&&key_code| {
            matches!(
                key_code,
                input::DOWN | input::UP | input::LEFT | input::RIGHT
            )
        })
        .unwrap()
}
