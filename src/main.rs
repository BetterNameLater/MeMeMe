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

use crate::level::components::level_to_go::LevelToGo;
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
        .add_systems(Startup, setup)
        .add_systems(Startup, loading_screen)
        .add_systems(OnExit(GameState::BootingGame), stop_loading_screen)
        // plugins
        .add_plugins(DefaultPlugins)
        .add_plugins(LevelPlugin)
        // assets
        .init_asset_loader::<MapLoader>()
        .init_asset::<MapRepr>()
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(LevelToGo("entry_point".to_string()));
}

#[derive(AssetCollection, Resource)]
struct LevelAssets {
    #[asset(
        paths(
            "levels/test_teleporter_is_teleporting.json",
            "levels/test_teleporter_is_teleporting_all_after_activation.json",
            "levels/test_pressure_plate_on_off.json",
            "levels/entry_point.json",
            "levels/example.json",
            "levels/example_2.json"
        ),
        collection(typed)
    )]
    levels: Vec<Handle<MapRepr>>,
}
