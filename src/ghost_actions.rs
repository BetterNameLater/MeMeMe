use crate::constantes::{
    INPUT_PLAYER_DOWN, INPUT_PLAYER_LEFT, INPUT_PLAYER_RIGHT, INPUT_PLAYER_UP,
};
use bevy::ecs::entity::Entity;
use bevy::input::Input;
use bevy::math::Vec3;
use bevy::prelude::{KeyCode, MouseButton, Res, Resource};

#[derive(Resource, Debug)]
pub struct GhostActions {
    pub list: Vec<Action>,
    pub index: usize,
}

pub fn actions_system(ghost_actions: Res<GhostActions>) {
    println!("{:?}", ghost_actions);
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
