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
use crate::time::{ElapsedTimeFromStartRewind, StartTime};
use bevy::{prelude::*, window::CursorGrabMode};
use map::*;

fn elapsed_time_from_start_rewind_system(
    mut elapsed_time_from_start_rewind: ResMut<ElapsedTimeFromStartRewind>,
    start_time: Res<StartTime>,
    time: Res<Time>,
) {
    if start_time.0.is_none() {
        return;
    }
    elapsed_time_from_start_rewind.0 = Some(time.elapsed_seconds() - start_time.0.unwrap());
}

fn main() {
    App::new()
        .insert_resource(State::default())
        .insert_resource(GhostActions::default())
        .insert_resource(StartTime(None))
        .insert_resource(ElapsedTimeFromStartRewind(None))
        .add_plugins((DefaultPlugins, PlayerPlugin))
        .add_systems(
            Update,
            (
                elapsed_time_from_start_rewind_system,
                cursor_grab_system,
                move_camera,
                ghost_actions_system,
                check_levels_loaded_system,
            ),
        ) // TODO: mettre un ordre
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

// fn setup_map(mut commands: Commands, custom_assets: Res<Assets<MapRepr>>, state: Res<State>) {
//
// }

fn cursor_grab_system(
    mut window: Query<&mut Window>,
    btn: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
) {
    let mut window = window.single_mut();

    if btn.just_pressed(MouseButton::Left) {
        window.cursor.visible = false;
        window.cursor.grab_mode = CursorGrabMode::Locked;
    }

    if key.just_pressed(KeyCode::Escape) {
        window.cursor.visible = true;
        window.cursor.grab_mode = CursorGrabMode::None;
    }
}

fn move_camera(
    mut cam_query: Query<&mut Transform, With<Camera>>,
    key_inputs: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    //Position
    let mut camera_pos = cam_query.single_mut();
    const SPEED: f32 = 1000.;

    let boost = if key_inputs.pressed(KeyCode::ControlLeft) {
        100.
    } else {
        1.
    };
    let move_value_x =
        f32::from(key_inputs.pressed(KeyCode::D)) - f32::from(key_inputs.pressed(KeyCode::A));
    let move_value_y =
        f32::from(key_inputs.pressed(KeyCode::W)) - f32::from(key_inputs.pressed(KeyCode::S));

    let movement = Vec3 {
        x: move_value_x,
        y: move_value_y,
        z: 0.,
    };
    camera_pos.translation += movement * time.delta_seconds() * SPEED * boost;

    if movement != Vec3::ZERO {
        println!("camera position : {:?}", camera_pos.translation);
    }
}
