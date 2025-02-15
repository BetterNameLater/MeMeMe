use crate::game_state::GameState;
use crate::level::components::level_tag::LevelTag;
use crate::level::ressources::level_informations::{GhostCount, PlayingTime, StartPosition};
use crate::player::actions::ActionStack;
use crate::player::components::player::Player;
use crate::player::Ghost;
use bevy::hierarchy::DespawnRecursiveExt;
use bevy::log::{debug, info};
use bevy::prelude::{Commands, Entity, NextState, Query, ResMut, With};

pub fn unload_level(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    level_query: Query<Entity, With<LevelTag>>,
) {
    info!("Unloading current level");

    commands.remove_resource::<ActionStack<Ghost>>();
    commands.remove_resource::<ActionStack<Player>>();
    commands.remove_resource::<GhostCount>();
    commands.remove_resource::<PlayingTime>();
    commands.remove_resource::<StartPosition>();

    next_state.set(GameState::LoadingLevel);

    let level = level_query.single();
    commands.entity(level).despawn_recursive();
    debug!("Current level recursively de-spawned");
}
