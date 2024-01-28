use bevy::prelude::KeyCode;

pub const PLAYER_Z: f32 = 1.0;

pub const CELL_LENGTH: f32 = 32.;
pub const CELL_LENGTH_USIZE: usize = CELL_LENGTH as usize;
pub const CELL_GAP: f32 = 8.;

// inputs
pub const INPUT_PLAYER_UP: KeyCode = KeyCode::Up;
pub const INPUT_PLAYER_DOWN: KeyCode = KeyCode::Down;
pub const INPUT_PLAYER_LEFT: KeyCode = KeyCode::Left;
pub const INPUT_PLAYER_RIGHT: KeyCode = KeyCode::Right;
pub const INPUT_PLAYER_REWIND: KeyCode = KeyCode::Space;

pub const INPUT_PLAYER_INTERACT: KeyCode = KeyCode::E;
