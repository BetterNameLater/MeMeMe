#![allow(dead_code, unused)]
#![allow(clippy::type_complexity)]
mod constantes;
mod items;
mod map;
mod map_parser;
mod math;
mod player;
mod time;

use crate::items::systems::is_activated::update_is_activated_system;
use crate::items::systems::people_on::count_people_on_system;
use crate::items::systems::teleport::{teleporter_activate_system, teleporter_system};
use crate::items::systems::toggle::{toggle_on_enter_system, toggle_on_interact_system};

use crate::constantes::PLAYER_START_TRANSFORM;
// use crate::items::{on_enter_system, PressurePlate};
use crate::items::ghost_only::GhostOnly;
use crate::items::is_usable::IsUsable;
use crate::items::player_only::PlayerOnly;
use crate::map_parser::{MapLoader, MapRepr};
use crate::math::vec2i::Vec2i;
use crate::player::player::{player_input_system, Player, PlayerPlugin};
use crate::player::{
    ghost_actions_system, Ghost, GhostActions, GhostNewPositionEvent, PlayerNewPositionEvent,
    RewindEvent,
};
use crate::time::{elapsed_time_from_start_rewind_system, ElapsedTimeFromStartRewind, StartTime};
use bevy::{prelude::*, window::CursorGrabMode};
use items::populate_items::populate_items;
use map::*;
use player::interact::{GhostInteractEvent, PlayerInteractEvent};
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
                count_people_on_system::<GhostOnly, PlayerNewPositionEvent>
                    .after(player_input_system),
                count_people_on_system::<PlayerOnly, GhostNewPositionEvent>
                    .after(player_input_system),
                teleporter_system::<PlayerOnly, GhostNewPositionEvent, Ghost>
                    .after(player_input_system),
                teleporter_system::<GhostOnly, PlayerNewPositionEvent, Player>
                    .after(player_input_system),
                teleporter_activate_system::<PlayerOnly, Ghost>.after(player_input_system),
                teleporter_activate_system::<GhostOnly, Player>.after(player_input_system),
                toggle_on_interact_system::<GhostOnly, PlayerInteractEvent>
                    .after(player_input_system),
                toggle_on_interact_system::<PlayerOnly, GhostInteractEvent>
                    .after(player_input_system),
                toggle_on_enter_system::<PlayerOnly, GhostNewPositionEvent>
                    .after(player_input_system),
                toggle_on_enter_system::<GhostOnly, PlayerNewPositionEvent>
                    .after(player_input_system),
                update_is_activated_system,
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

    let mut world_map = Map::default();

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
                    world_map.spawn_cell(
                        &mut commands,
                        Vec2i {
                            x: x as i32,
                            y: y as i32,
                        },
                        background_type,
                    )
                })
        });
    commands.spawn((world_map, WorldMap));
    let items = populate_items(&mut commands, &level_example.objects);
    let object_map = Map { cells: items };
    commands.spawn((object_map, ObjectMap));
}
