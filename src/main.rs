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
        .add_plugins((DefaultPlugins, PlayerPlugin))
        .init_asset::<MapRepr>()
        .init_asset_loader::<MapLoader>()
        .add_systems(
            Update,
            (
                elapsed_time_from_start_rewind_system,
                cursor_grab_system,
                move_camera,
                ghost_actions_system,
            ),
        ) // TODO: mettre un ordre
        .add_systems(PreStartup, load_levels)
        .add_systems(Startup, setup)
        .init_resource::<Handlerrs>()
        .insert_resource(GhostActions {
            actions: vec![],
            index: 0,
        })
        .insert_resource(StartTime(None))
        .insert_resource(ElapsedTimeFromStartRewind(None))
        .run();
}

#[derive(Resource, Default)]
struct Handlerrs(Handle<MapRepr>);

fn load_levels(mut h: ResMut<Handlerrs>, asset_server: Res<AssetServer>) {
    h.0 = asset_server.load("levels/example.json");
}

fn setup(mut commands: Commands, custom_assets: ResMut<Assets<MapRepr>>, h: Res<Handlerrs>) {
    // let level_example = custom_assets.(&h.0).unwrap();

    let mut map = Map::default();
    for x in 0..30 {
        for y in 0..30 {
            map.spawn_cell(&mut commands, Vec2i { x, y })
        }
    }
    commands.spawn(Camera2dBundle::default());
    commands.spawn(map);
}

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
