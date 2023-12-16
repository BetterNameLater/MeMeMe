use bevy::prelude::{KeyCode, Transform, Vec3};

pub const PLAYER_Z: f32 = 1.0;
pub const PLAYER_START_TRANSFORM: Transform = Transform::from_xyz(0., 0., PLAYER_Z);

pub const CELL_LENGTH: f32 = 32.;
pub const CELL_GAP: f32 = 8.;

// inputs
pub const INPUT_PLAYER_UP: KeyCode = KeyCode::Up;
pub const INPUT_PLAYER_DOWN: KeyCode = KeyCode::Down;
pub const INPUT_PLAYER_LEFT: KeyCode = KeyCode::Left;
pub const INPUT_PLAYER_RIGHT: KeyCode = KeyCode::Right;
pub const INPUT_PLAYER_REWIND: KeyCode = KeyCode::Space;
