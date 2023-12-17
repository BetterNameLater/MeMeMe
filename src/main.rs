#![allow(dead_code, unused)]
mod constantes;
mod map;
mod map_parser;
mod math;
mod player;
mod time;

use crate::map_parser::{MapLoader, MapRepr};
use crate::math::vec2i::Vec2i;
use crate::player::{ghost_actions_system, GhostActions, PlayerPlugin};
use crate::time::{elapsed_time_from_start_rewind_system, ElapsedTimeFromStartRewind, StartTime};
use bevy::ecs::schedule::{ScheduleBuildSettings, ScheduleLabel};
use bevy::{prelude::*, window::CursorGrabMode};
use map::*;

fn main() {
    App::new()
        .insert_resource(State::default())
        .insert_resource(GhostActions::default())
        .insert_resource(StartTime(None))
        .insert_resource(ElapsedTimeFromStartRewind(None))
        .add_plugins((DefaultPlugins, PlayerPlugin))
        .add_systems(SpawnScene, check_levels_loaded_system)
        .add_systems(
            Update,
            (elapsed_time_from_start_rewind_system, ghost_actions_system),
        )
        .add_systems(Startup, load_levels)
        .init_asset_loader::<MapLoader>()
        .init_asset::<MapRepr>()
        .run();
}

#[derive(Resource, Default)]
struct State {
    level_1_handle: Option<Handle<MapRepr>>,
    levels_loaded: bool,
}

fn load_levels(mut commands: Commands, asset_server: Res<AssetServer>, mut state: ResMut<State>) {
    commands.spawn(Camera2dBundle::default());
    state.level_1_handle = Some(asset_server.load("levels/example.json"));
}

fn check_levels_loaded_system(
    mut commands: Commands,
    mut state: ResMut<State>,
    custom_assets: Res<Assets<MapRepr>>,
) {
    if state.levels_loaded {
        return;
    }

    let level_example = custom_assets.get(state.level_1_handle.as_ref().unwrap());
    if level_example.is_none() {
        return;
    }
    state.levels_loaded = true;
    let level_example = custom_assets
        .get(state.level_1_handle.as_ref().unwrap())
        .unwrap();

    let mut map = Map::default();

    level_example
        .map
        .iter()
        .rev()
        .enumerate()
        .for_each(|(y, map_slice)| {
            map_slice
                .iter()
                .enumerate()
                .for_each(|(x, background_type)| {
                    map.spawn_cell(
                        &mut commands,
                        Vec2i {
                            x: x as i32,
                            y: y as i32,
                        },
                        background_type,
                    )
                })
        });
    commands.spawn(map);
}
