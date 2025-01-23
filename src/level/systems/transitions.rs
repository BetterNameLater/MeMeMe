use crate::{
    game_state::GameState,
    level::{components::level_to_go::LevelToGo, level_state::LevelState},
};
use bevy::prelude::*;

pub fn enter_rewind(mut next_state: ResMut<NextState<LevelState>>) {
    next_state.set(LevelState::Idle);
}

pub fn enter_won(mut commands: Commands, mut next_state: ResMut<NextState<GameState>>) {
    commands.insert_resource(LevelToGo("entry_point".to_string()));
    next_state.set(GameState::LoadingLevel);
}
