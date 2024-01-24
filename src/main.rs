#![allow(dead_code, unused)]
#![allow(clippy::type_complexity)]
mod constantes;
mod items;
mod level;
mod map;
mod map_parser;
mod math;
mod player;
mod state;
mod time;

use crate::items::enterable::people_enter_system;
use crate::items::events::OnEnterEvent;
use crate::items::ghost_only::GhostOnly;
use crate::items::level_teleporter::level_teleporter_system;
use crate::items::player_only::PlayerOnly;
use crate::items::systems::is_activated::update_is_activated_system;
use crate::items::systems::people_on::count_people_on_system;
use crate::items::systems::teleport::{teleporter_activate_system, teleporter_system};
use crate::items::systems::toggle::{toggle_on_enter_system, toggle_on_interact_system};
use crate::level::plugin::LevelPlugin;
use crate::map_parser::{MapLoader, MapRepr};
use crate::math::vec2i::Vec2i;
use crate::menu::loading_screen::{loading_screen, stop_loading_screen};
use crate::player::events::RewindEvent;
use crate::player::player::{player_input_system, Player, PlayerPlugin};
use crate::player::{
    ghost_actions_system, Ghost, GhostActions, GhostNewPositionEvent, PlayerNewPositionEvent,
};
use crate::time::{elapsed_time_from_start_rewind_system, ElapsedTimeFromStartRewind, StartTime};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use items::populate_items::populate_items;
use map::*;
use player::interact::{GhostInteractEvent, PlayerInteractEvent};
use state::GameState;

fn main() {
    App::new()
        // states
        .add_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::BootingGame)
                .continue_to_state(GameState::LoadingLevel)
                .load_collection::<LevelAssets>(),
        )
        // systems
        .add_systems(PreStartup, loading_screen)
        .add_systems(OnExit(GameState::BootingGame), stop_loading_screen)
        .add_systems(OnEnter(GameState::LoadingLevel), load_level)
        // plugins
        .add_plugins(DefaultPlugins)
        .add_plugins(LevelPlugin)
        // assets
        .init_asset_loader::<MapLoader>()
        .init_asset::<MapRepr>()
        .run();
}

#[derive(AssetCollection, Resource)]
struct LevelAssets {
    #[asset(path = "levels/entry_point.json")]
    entry_point: Handle<MapRepr>,
}

fn load_level(
    mut commands: Commands,
    mut level_assets: Res<LevelAssets>,
    custom_assets: Res<Assets<MapRepr>>,
    mut game_state: ResMut<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    commands.spawn(Camera2dBundle::default());
    let level = custom_assets.get(level_assets.entry_point.clone()).unwrap();
    let mut world_map = Map::default();
    level
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
    let items = populate_items(&mut commands, &level.objects);
    let object_map = Map { cells: items };
    commands.spawn((object_map, ObjectMap));
    next_state.set(GameState::InLevel);
}

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app
            // resources
            .insert_resource(GhostActions::default())
            .insert_resource(StartTime(None))
            .insert_resource(ElapsedTimeFromStartRewind(None))
            // plugins
            .add_plugins((DefaultPlugins, PlayerPlugin))
            // events
            .add_event::<OnEnterEvent>()
            // systems
            .add_systems(
                Update,
                (
                    elapsed_time_from_start_rewind_system,
                    people_enter_system::<PlayerOnly, GhostNewPositionEvent>
                        .after(player_input_system),
                    people_enter_system::<GhostOnly, PlayerNewPositionEvent>
                        .after(player_input_system),
                    level_teleporter_system
                        .after(people_enter_system::<PlayerOnly, GhostNewPositionEvent>),
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
                )
                    .run_if(in_state(GameState::InLevel)),
            );
    }
}
