#![allow(dead_code, unused)]
#![allow(clippy::type_complexity)]
mod constantes;
mod items;
mod level;
mod map;
mod map_parser;
mod math;
mod menu;
mod player;
mod state;
mod time;

use crate::items::enterable::{people_enter_system, EnterAble};
use crate::items::events::OnEnterEvent;
use crate::items::ghost_only::GhostOnly;
use crate::items::level_teleporter::level_teleporter_system;
use crate::items::player_only::PlayerOnly;
use crate::items::systems::is_activated::update_is_activated_system;
use crate::items::systems::people_on::count_people_on_system;
use crate::items::systems::teleport::{teleporter_activate_system, teleporter_system};
use crate::items::systems::toggle::{toggle_on_enter_system, toggle_on_interact_system};
use crate::level::load_level::load_level;
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
use bevy::math::Affine3A;
use bevy::prelude::*;
use bevy::winit::WinitSettings;
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
    #[asset(
        paths("levels/example.json", "levels/example_2.json"),
        collection(typed)
    )]
    levels: Vec<Handle<MapRepr>>,

    #[asset(path = "levels/entry_point.json")]
    entry_point: Handle<MapRepr>,
}
