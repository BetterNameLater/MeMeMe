use super::math::vec2i::Vec2i;
use crate::map::Map;
use bevy::{prelude::*, utils::HashMap};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_player_system)
            .add_systems(Update, player_control_system);
    }
}

fn create_player_system(mut commands: Commands, map_query: Query<&Map>) {
    let map = map_query.single();
    let size = map.get_cell_length() / 2.;
    // let transform =

    commands.spawn((
        Player,
        SpriteBundle {
            sprite: Sprite {
                color: Color::BEIGE,
                custom_size: Some(Vec2::new(size, size)),
                ..default()
            },
            transform: Transform::from_xyz(0., 0., 0.1),
            global_transform: Default::default(),
            texture: Default::default(),
            visibility: Default::default(),
            inherited_visibility: Default::default(),
            view_visibility: Default::default(),
        },
    ));
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Ghost;

#[derive(Component)]
pub struct MapPosition(Vec2i);

const PLAYER_UP: KeyCode = KeyCode::Up;
const PLAYER_DOWN: KeyCode = KeyCode::Down;
const PLAYER_LEFT: KeyCode = KeyCode::Left;
const PLAYER_RIGHT: KeyCode = KeyCode::Right;

fn player_control_system(
    mut player_query: Query<&mut Transform, With<Player>>,
    key_inputs: Res<Input<KeyCode>>,
) {
    let mut player_position = player_query.single_mut();
    let mut movement = Vec3::ZERO;

    let move_key = key_inputs
        .get_just_pressed()
        .find(|key_code| match **key_code {
            PLAYER_DOWN | PLAYER_UP | PLAYER_LEFT | PLAYER_RIGHT => true,
            _ => false,
        });
    if let Some(move_key) = move_key {
        player_position.translation += 32.
            * match *move_key {
                PLAYER_UP => Vec3::Y,
                PLAYER_DOWN => Vec3::NEG_Y,
                PLAYER_LEFT => Vec3::NEG_X,
                PLAYER_RIGHT => Vec3::X,
                _ => unreachable!(),
            };
    }
}
