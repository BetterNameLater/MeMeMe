#![allow(dead_code, unused)]
#![allow(clippy::type_complexity)]
mod constantes;
mod items;
mod map;
mod map_parser;
mod math;
mod player;
mod time;

use crate::constantes::PLAYER_START_TRANSFORM;
// use crate::items::{on_enter_system, PressurePlate};
use crate::items::is_on::IsOn;
use crate::items::people_on::{
    count_people_on_ghost_only_system, count_people_on_player_only_system, count_people_on_system,
    PeopleOn,
};
use crate::map_parser::{MapLoader, MapRepr};
use crate::math::vec2i::Vec2i;
use crate::player::player::{player_input_system, PlayerPlugin};
use crate::player::{ghost_actions_system, GhostActions, RewindEvent};
use crate::time::{elapsed_time_from_start_rewind_system, ElapsedTimeFromStartRewind, StartTime};
use bevy::{prelude::*, window::CursorGrabMode};
use items::populate_items::populate_items;
use map::*;
use std::any::Any;

fn main() {
    App::new()
        // resources
        .insert_resource(State::default())
        .insert_resource(GhostActions::default())
        .insert_resource(StartTime(None))
        .insert_resource(ElapsedTimeFromStartRewind(None))
        // systems
        .add_systems(Startup, load_levels)
        .add_systems(SpawnScene, check_levels_loaded_system)
        .add_plugins((DefaultPlugins, PlayerPlugin))
        .add_systems(
            Update,
            (
                elapsed_time_from_start_rewind_system,
                count_people_on_system.after(player_input_system),
                count_people_on_ghost_only_system.after(player_input_system),
                count_people_on_player_only_system.after(player_input_system),
            ),
        )
        // assets
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
    commands.spawn((map, WorldMap));
    // TODO by the albaud
    //  commands.spawn((object_map, ObjectMap));
    let items = populate_items(commands, &level_example.objects);
}
