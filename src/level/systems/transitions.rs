use crate::level::level_state::LevelState;
use bevy::prelude::*;

pub fn enter_rewind(mut next_state: ResMut<NextState<LevelState>>) {
    next_state.set(LevelState::Idle);
}
