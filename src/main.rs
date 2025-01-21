#![allow(clippy::type_complexity)]
mod constantes;
mod game_state;
mod items;
mod level;
mod map;
mod menu;
mod player;

use crate::level::components::level_to_go::LevelToGo;
use crate::level::plugin::LevelPlugin;
use crate::menu::loading_screen::{error_screen, loading_screen, unload_message_screen};
use bevy::log::{Level, LogPlugin};
use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_asset_loader::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use game_state::GameState;
use level_parser::MapLoader;
use level_parser::WorldRepr;

fn main() {
    let mut app = App::new();

    app
        // systems
        .add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Camera2d);
        })
        .add_systems(Startup, loading_screen)
        .add_systems(OnExit(GameState::BootingGame), unload_message_screen)
        .add_systems(OnEnter(GameState::ErrorInitialLoad), error_screen)
        .add_systems(
            // we don't have any menu yet, so load direct hub level
            OnEnter(GameState::Menu),
            |mut commands: Commands, mut next_state: ResMut<NextState<GameState>>| {
                commands.insert_resource(LevelToGo("entry_point".to_string()));
                next_state.set(GameState::LoadingLevel);
            },
        )
        // plugins
        .add_plugins(
            DefaultPlugins
                .set(LogPlugin {
                    level: Level::INFO,
                    filter: "wgpu=error,bevy_render=info,bevy_ecs=info,me_me_me=trace".to_string(),
                    ..default()
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
        // assets
        .init_asset_loader::<MapLoader>()
        .init_asset::<WorldRepr>()
        // states
        .init_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::BootingGame)
                .continue_to_state(GameState::Menu)
                .on_failure_continue_to_state(GameState::ErrorInitialLoad)
                .load_collection::<LevelAssets>(),
        );

    #[cfg(debug_assertions)]
    app.add_plugins(WorldInspectorPlugin::new())
        .add_systems(Update, log_transitions::<GameState>);

    app.run();
}

#[derive(AssetCollection, Resource)]
struct LevelAssets {
    #[asset(path = "levels.ldtk")]
    pub world: Handle<WorldRepr>,
}

#[cfg(debug_assertions)]
/// print when an Event transition happens
pub fn log_transitions<T: States>(mut transitions: EventReader<StateTransitionEvent<T>>) {
    for transition in transitions.read() {
        info!(
            "transition: {:?} => {:?}",
            transition.exited, transition.entered
        );
    }
}
