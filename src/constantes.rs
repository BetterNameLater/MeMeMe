#![allow(unused)]

pub const CELL_Z: f32 = 1.0;
pub const FLAG_Z: f32 = 2.0;
pub const ITEMS_Z: f32 = 4.0;
pub const PLAYER_Z: f32 = 5.0;

pub const CELL_LENGTH: f32 = 32.;
pub const ITEM_LENGHT: f32 = CELL_LENGTH / 2.;
pub const CELL_LENGTH_USIZE: usize = CELL_LENGTH as usize;
pub const CELL_GAP: f32 = 8.;

pub mod input {
    use bevy::prelude::KeyCode;

    pub const UP: KeyCode = KeyCode::ArrowUp;
    pub const DOWN: KeyCode = KeyCode::ArrowDown;
    pub const LEFT: KeyCode = KeyCode::ArrowLeft;
    pub const RIGHT: KeyCode = KeyCode::ArrowRight;
    pub const REWIND: KeyCode = KeyCode::Space;
    pub const INTERACT: KeyCode = KeyCode::KeyE;
    pub const RESET: KeyCode = KeyCode::KeyR;

    #[cfg(debug_assertions)]
    pub const HOME_DEBUG: KeyCode = KeyCode::KeyH;
}
