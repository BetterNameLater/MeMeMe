#![allow(clippy::type_complexity)]
mod collision;
mod constantes;
mod items;
mod level;
mod map;
mod map_parser;
mod math {
    pub mod vec2i;
}
mod menu;
mod player;
mod state;

use crate::collision::plugin::CollisionPlugin;
use crate::level::components::level_to_go::LevelToGo;
use crate::level::plugin::LevelPlugin;
use crate::level::ressources::level_informations::LevelInformations;
use crate::map_parser::{MapLoader, MapRepr};
use crate::menu::loading_screen::{error_screen, loading_screen, unload_message_screen};
use crate::player::GhostActions;
use bevy::log::{Level, LogPlugin};
use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_asset_loader::prelude::*;
use bevy_inspector_egui::quick::{ResourceInspectorPlugin, WorldInspectorPlugin};
use state::GameState;

fn main() {
    App::new()
        // states
        .add_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::BootingGame)
                .continue_to_state(GameState::LoadingLevel)
                .on_failure_continue_to_state(GameState::ErrorInitialLoad)
                .load_collection::<LevelAssets>(),
        )
        // systems
        .add_systems(Startup, setup)
        .add_systems(Startup, loading_screen)
        .add_systems(OnExit(GameState::BootingGame), unload_message_screen)
        .add_systems(OnEnter(GameState::ErrorInitialLoad), error_screen)
        // plugins
        .add_plugins(
            DefaultPlugins
                .set(LogPlugin {
                    level: Level::INFO,
                    filter: "wgpu=error,bevy_render=info,bevy_ecs=info,me_me_me=trace".to_string(),
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(1600., 900.),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(LevelPlugin)
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(ResourceInspectorPlugin::<LevelInformations>::default())
        .add_plugins(ResourceInspectorPlugin::<GhostActions>::default())
        .add_plugins(CollisionPlugin)
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
    #[asset(path = "levels", collection(typed))]
    levels: Vec<Handle<MapRepr>>,
}
