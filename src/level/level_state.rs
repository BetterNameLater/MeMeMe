use crate::game_state::GameState;
use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(GameState = GameState::InLevel)]
pub enum LevelState {
    /// Before player enter any input after start or rewind
    #[default]
    Idle,
    /// Player did move, and the game runs
    Playing,
    /// One tick state, the player pressed the rewind button, it will rewind and transition to [`LevelState::Idle`] next
    Rewind,
}
