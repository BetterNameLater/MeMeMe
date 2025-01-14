use bevy::prelude::KeyCode;

pub const CELL_Z: f32 = 1.0;
pub const ITEMS_Z: f32 = 4.0;
pub const PLAYER_Z: f32 = 5.0;

pub const CELL_LENGTH: f32 = 32.;
pub const CELL_LENGTH_USIZE: usize = CELL_LENGTH as usize;
pub const CELL_GAP: f32 = 8.;

// inputs
pub const INPUT_PLAYER_UP: KeyCode = KeyCode::ArrowUp;
pub const INPUT_PLAYER_DOWN: KeyCode = KeyCode::ArrowDown;
pub const INPUT_PLAYER_LEFT: KeyCode = KeyCode::ArrowLeft;
pub const INPUT_PLAYER_RIGHT: KeyCode = KeyCode::ArrowRight;
pub const INPUT_PLAYER_REWIND: KeyCode = KeyCode::Space;

pub const INPUT_PLAYER_INTERACT: KeyCode = KeyCode::KeyE;
