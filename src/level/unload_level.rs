use crate::game_state::GameState;
use crate::level::components::level_tag::LevelTag;
use crate::level::components::level_to_go::LevelToGo;
use crate::level::ressources::level_informations::{GhostCount, LevelInformations, StartPosition};
use crate::map_parser::MapRepr;
use crate::player::GhostActions;
use crate::LevelAssets;
use bevy::asset::Assets;
use bevy::hierarchy::DespawnRecursiveExt;
use bevy::log::{debug, info};
use bevy::prelude::{Commands, Entity, NextState, Query, Res, ResMut, With};

pub fn unload_level(
    mut commands: Commands,
    _level_assets: Res<LevelAssets>,
    _custom_assets: Res<Assets<MapRepr>>,
    mut next_state: ResMut<NextState<GameState>>,
    level_query: Query<Entity, With<LevelTag>>,
    level_to_go_query: Query<&LevelToGo>,
) {
    info!("Unloading current level");

    commands.remove_resource::<GhostActions>();
    commands.remove_resource::<GhostCount>();
    commands.remove_resource::<LevelInformations>();
    commands.remove_resource::<StartPosition>();

    // TODO pas tres elegant de verifier ca
    if level_to_go_query.get_single().is_ok() {
        next_state.set(GameState::LoadingLevel);
    } else {
        // TODO
    }
    let level = level_query.single();
    commands.entity(level).despawn_recursive();
    debug!("Current level recursively de-spawned");
}
