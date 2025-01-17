use crate::constantes::PLAYER_Z;
use crate::game_state::GameState;
use crate::level::components::level_tag::LevelTag;
use crate::level::level_state::LevelState;
use crate::level::ressources::level_informations::{GhostCount, PlayingTime, StartPosition};
use crate::player::actions::ActionStack;
use crate::player::components::player::Player;
use crate::player::Ghost;
use bevy::hierarchy::DespawnRecursiveExt;
use bevy::log::{debug, info};
use bevy::prelude::{Commands, Entity, NextState, Query, Res, ResMut, Transform, With};

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

pub fn reset_level(
    mut commands: Commands,
    mut next_state: ResMut<NextState<LevelState>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    start_position: Res<StartPosition>,
    ghosts_query: Query<Entity, With<Ghost>>,
) {
    info!("reseting current level");

    commands.remove_resource::<ActionStack<Ghost>>();
    commands.remove_resource::<ActionStack<Player>>();
    commands.remove_resource::<GhostCount>();
    commands.remove_resource::<PlayingTime>();

    commands.init_resource::<ActionStack<Ghost>>();
    commands.init_resource::<ActionStack<Player>>();
    commands.init_resource::<GhostCount>();

    let mut player_transform = player_query.single_mut();
    let start_transform = start_position.get().to_transform(PLAYER_Z);
    *player_transform = start_transform;

    ghosts_query
        .iter()
        .for_each(|id| commands.entity(id).despawn());
    next_state.set(LevelState::Idle);
}
