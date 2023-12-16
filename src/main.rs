#![allow(dead_code, unused)]
mod constantes;
mod ghost_actions;
mod map;
mod math;
mod player;

use crate::ghost_actions::{actions_system, GhostActions};
use crate::math::vec2i::Vec2i;
use crate::player::PlayerPlugin;
use bevy::{pbr::DirectionalLightShadowMap, prelude::*, window::CursorGrabMode};
use map::*;

#[derive(Resource)]
struct StartTime(pub Option<f32>);

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MapPlugin, PlayerPlugin))
        .add_systems(Update, (cursor_grab_system, move_camera, actions_system))
        .add_systems(Startup, setup)
        .insert_resource(DirectionalLightShadowMap { size: 1000 })
        .insert_resource(GhostActions {
            list: vec![],
            index: 0,
        })
        .insert_resource(StartTime(None))
        .run();
}

fn setup(mut commands: Commands, mut map_query: Query<&mut Map>) {
    commands.spawn(Camera2dBundle::default());

    let mut map = map_query.single_mut();
    for i in 0..30 {
        for j in 0..30 {
            map.spawn_cell(&mut commands, Vec2i { x: i as i32, y: j })
        }
    }
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
    let move_value_z = f32::from(key_inputs.pressed(KeyCode::Space))
        - f32::from(key_inputs.pressed(KeyCode::ShiftLeft));

    let movement = Vec3 {
        x: move_value_x,
        y: move_value_y,
        z: move_value_z,
    };
    camera_pos.translation += movement * time.delta_seconds() * SPEED * boost;

    if movement != Vec3::ZERO {
        println!("{:?}", camera_pos.translation);
    }
}
