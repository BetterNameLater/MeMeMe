use crate::game_state::GameState;
use crate::level::components::level_tag::LevelTag;
use crate::level::ressources::level_informations::{GhostCount, PlayingTime, StartPosition};
use crate::player::actions::ActionStack;
use crate::player::Ghost;
use crate::LevelAssets;
use bevy::asset::Assets;
use bevy::hierarchy::DespawnRecursiveExt;
use bevy::log::{debug, info};
use bevy::prelude::{Commands, Entity, NextState, Query, Res, ResMut, With};
use level_parser::MapRepr;

pub fn unload_level(
    mut commands: Commands,
    _level_assets: Res<LevelAssets>,
    _custom_assets: Res<Assets<MapRepr>>,
    mut next_state: ResMut<NextState<GameState>>,
    level_query: Query<Entity, With<LevelTag>>,
) {
    info!("Unloading current level");

    commands.remove_resource::<ActionStack<Ghost>>();
    commands.remove_resource::<GhostCount>();
    commands.remove_resource::<PlayingTime>();
    commands.remove_resource::<StartPosition>();

    next_state.set(GameState::LoadingLevel);

    let level = level_query.single();
    commands.entity(level).despawn_recursive();
    debug!("Current level recursively de-spawned");
}
