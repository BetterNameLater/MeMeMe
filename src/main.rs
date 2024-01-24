// #![allow(dead_code, unused)]
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

use crate::level::load_level::load_level;
use crate::level::plugin::LevelPlugin;
use crate::map_parser::{MapLoader, MapRepr};
use crate::menu::loading_screen::{loading_screen, stop_loading_screen};
use crate::time::{ElapsedTimeFromStartRewind, StartTime};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
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
