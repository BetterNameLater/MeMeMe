use crate::constantes::{
    CELL_LENGTH, INPUT_PLAYER_DOWN, INPUT_PLAYER_LEFT, INPUT_PLAYER_RIGHT, INPUT_PLAYER_UP,
};
use crate::ghost::{self, Ghost};
use crate::{ElapsedTimeFromStartRewind, StartTime};
use bevy::ecs::entity::Entity;
use bevy::ecs::query::With;
use bevy::ecs::system::Query;
use bevy::input::Input;
use bevy::math::Vec3;
use bevy::prelude::{KeyCode, MouseButton, Res, ResMut, Resource};
use bevy::time::Time;
use bevy::transform::components::Transform;

#[derive(Resource, Debug)]
pub struct GhostActions {
    pub list: Vec<Action>,
    pub index: usize,
}

pub fn actions_system(
    time: Res<Time>,
    start_time: Res<StartTime>,
    mut ghost_actions: ResMut<GhostActions>,
    mut ghosts_query: Query<&mut Transform, With<Ghost>>,
    mut elapsed_time_from_start_rewind: ResMut<ElapsedTimeFromStartRewind>,
) {
    println!("{:?}", ghost_actions);
    if let Some(current_time) = elapsed_time_from_start_rewind.0 {
        loop {
            if ghost_actions.index >= ghost_actions.list.len() {
                return;
            }
            let Action {
                ghost_id,
                timestamp_seconds: action_time,
                action_type,
            } = &ghost_actions.list[ghost_actions.index];
            if action_time > &current_time {
                return;
            }
            println!("Action: {}, current_time {}", action_time, current_time);
            match action_type {
                ActionType::Move(move_direction) => {
                    let direction = move_direction.to_vec3();
                    let mut ghost_transform = ghosts_query.get_mut(*ghost_id).unwrap();
                    ghost_transform.translation += direction * CELL_LENGTH;
                }
            }
            ghost_actions.index += 1;
        }
    } else {
        return;
    }
}

#[derive(Debug)]
pub struct Action {
    pub ghost_id: Entity,
    pub timestamp_seconds: f32,
    pub action_type: ActionType,
}

#[derive(Debug)]

pub enum ActionType {
    Move(MoveDirection),
    // Levier
    // TODO
}

#[derive(Debug)]

pub enum MoveDirection {
    Up,
    Left,
    Down,
    Right,
}

impl MoveDirection {
    pub fn from_key_code(key_code: KeyCode) -> Self {
        match key_code {
            INPUT_PLAYER_UP => Self::Up,
            INPUT_PLAYER_DOWN => Self::Down,
            INPUT_PLAYER_LEFT => Self::Left,
            INPUT_PLAYER_RIGHT => Self::Right,
            _ => unreachable!(),
        }
    }

    pub fn to_vec3(&self) -> Vec3 {
        match self {
            MoveDirection::Up => Vec3::Y,
            MoveDirection::Left => Vec3::NEG_X,
            MoveDirection::Down => Vec3::NEG_Y,
            MoveDirection::Right => Vec3::X,
        }
    }
}
