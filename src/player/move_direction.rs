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
            input::UP => Self::Up,
            input::DOWN => Self::Down,
            input::LEFT => Self::Left,
            input::RIGHT => Self::Right,
            _ => unreachable!(),
        }
    }

    pub fn to_vec3(self) -> Vec3 {
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
            MoveDirection::Up => input::UP,
            MoveDirection::Left => input::LEFT,
            MoveDirection::Down => input::DOWN,
            MoveDirection::Right => input::RIGHT,
        }
    }
}

impl From<&MoveDirection> for KeyCode {
    fn from(val: &MoveDirection) -> Self {
        (*val).into()
    }
}
