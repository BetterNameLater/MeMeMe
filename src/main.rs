use bevy::{prelude::*, input::mouse::MouseMotion, window::CursorGrabMode, pbr::DirectionalLightShadowMap, sprite::MaterialMesh2dBundle};
mod map;
use map::*;
fn main() {
    App::new()
    .add_plugins((DefaultPlugins, MapPlugin))
    .add_systems(Update, (cursor_grab_system))
    .add_systems(Startup, setup)
    .insert_resource(DirectionalLightShadowMap { size: 1000 })
    .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut map_query: Query<&mut Map>
) {
    commands.spawn(Camera2dBundle::default());

    let mut map = map_query.single_mut();
    for i in 0..3 {
        for j in 0..30 {
            let cell_pos = map.map_to_local(Vec2i::from(i, j));
            commands.spawn((SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.25, 0.25, 0.75),
                    custom_size: Some(Vec2::new(50.0, 50.0)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(cell_pos.x, cell_pos.y, 0.)),
                ..default()
            }, Cell));
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
    mut mouse_motion: EventReader<MouseMotion>,
    time: Res<Time>,
    window: Query<&Window>,
) {
    //Position
    let mut camera_pos = cam_query.single_mut();
    const SPEED: f32 = 100.;
    const ROTATION_SPEED: f32 = 3.;
    
    let boost = if key_inputs.pressed(KeyCode::ControlLeft) {100.} else {1.};
    let move_value_x =  f32::from(key_inputs.pressed(KeyCode::D)) - f32::from(key_inputs.pressed(KeyCode::A));
    let move_value_y =  f32::from(key_inputs.pressed(KeyCode::Space)) - f32::from(key_inputs.pressed(KeyCode::ShiftLeft)) ;
    let move_value_z =  f32::from(key_inputs.pressed(KeyCode::S)) - f32::from(key_inputs.pressed(KeyCode::W));

    let z_rotation = Quat::from_xyzw(0., camera_pos.rotation.y, 0., camera_pos.rotation.w).normalize();
    let direction = z_rotation.mul_vec3(Vec3 { x: move_value_x, y: move_value_y, z: move_value_z }).normalize_or_zero();

    camera_pos.translation += Vec3 {x: move_value_x, y: move_value_y, z: move_value_z} * time.delta_seconds() * SPEED * boost;

    //Rotation
    let mut rotation_vec = Vec2::ZERO;
    for ev in mouse_motion.read() {
        rotation_vec += ev.delta;
    }

    let window = window.single();
    camera_pos.rotate_local_x(-rotation_vec.y / window.height() * ROTATION_SPEED);
    camera_pos.rotate_y(-rotation_vec.x / window.width() * ROTATION_SPEED);
    if direction != Vec3::ZERO {
        println!("{:?}", camera_pos.translation);
    }
}