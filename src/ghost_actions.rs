use crate::constantes::{
    INPUT_PLAYER_DOWN, INPUT_PLAYER_LEFT, INPUT_PLAYER_RIGHT, INPUT_PLAYER_UP,
};
use crate::ghost::{self, Ghost};
use crate::StartTime;
use bevy::ecs::entity::Entity;
use bevy::ecs::query::With;
use bevy::ecs::system::Query;
use bevy::input::Input;
use bevy::math::Vec3;
use bevy::prelude::{KeyCode, MouseButton, Res, Resource};
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
    ghost_actions: Res<GhostActions>,
    mut ghosts_query: Query<&mut Transform, With<Ghost>>,
) {
    /*     println!("{:?}", ghost_actions); */
    if let Some(start) = start_time.0 {
        let current_time = time.elapsed_seconds() - start;
        loop {
            if ghost_actions.index >= ghost_actions.list.len() {
                return;
            }
            let Action {
                ghost_id,
                timestamp_seconds: action_time,
                action_type,
            } = &ghost_actions.list[ghost_actions.index];
            if action_time < &current_time {
                return;
            }
        }
    } else {
        return;
    }

    for ghost in ghosts_query.iter_mut() { /*         println!("Ghost position : {:?}", ghost); */ }
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
