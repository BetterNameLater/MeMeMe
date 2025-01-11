use crate::constantes::*;
use bevy::math::Vec3;
use bevy::prelude::{KeyCode, Reflect};

#[derive(Debug, Reflect, PartialEq, Clone, Copy)]
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

impl From<MoveDirection> for KeyCode {
    fn from(val: MoveDirection) -> Self {
        match val {
            MoveDirection::Up => INPUT_PLAYER_UP,
            MoveDirection::Left => INPUT_PLAYER_LEFT,
            MoveDirection::Down => INPUT_PLAYER_DOWN,
            MoveDirection::Right => INPUT_PLAYER_RIGHT,
        }
    }
}

impl From<&MoveDirection> for KeyCode {
    fn from(val: &MoveDirection) -> Self {
        (*val).into()
    }
}
