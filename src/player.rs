use super::math::vec2i::Vec2i;
use crate::constantes::{
    CELL_LENGTH, INPUT_PLAYER_DOWN, INPUT_PLAYER_LEFT, INPUT_PLAYER_RIGHT, INPUT_PLAYER_UP,
    PLAYER_Z,
};
use crate::ghost_actions::{Action, ActionType, MoveDirection};
use crate::map::Map;
use crate::StartTime;
use bevy::{prelude::*, utils::HashMap};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_player_system)
            .add_systems(Update, player_control_system);
    }
}

fn create_player_system(mut commands: Commands) {
    let size = CELL_LENGTH / 2.;
    commands.spawn((
        Player::default(),
        SpriteBundle {
            sprite: Sprite {
                color: Color::BEIGE,
                custom_size: Some(Vec2::new(size, size)),
                ..default()
            },
            transform: Transform::from_xyz(0., 0., PLAYER_Z),
            ..default()
        },
    ));
}

#[derive(Component, Default)]
pub struct Player {
    actions: Vec<Action>,
}

#[derive(Component)]
pub struct Ghost;

#[derive(Component)]
pub struct MapPosition(Vec2i);

fn player_control_system(
    mut player_trans_query: Query<&mut Transform, With<Player>>,
    mut player_query: Query<&mut Player>,
    key_inputs: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut start_time: ResMut<StartTime>,
) {
    let mut player_position = player_trans_query.single_mut();
    let mut player = player_query.single_mut();
    let mut movement = Vec3::ZERO;

    let move_key = key_inputs
        .get_just_pressed()
        .find(|key_code| match **key_code {
            INPUT_PLAYER_DOWN | INPUT_PLAYER_UP | INPUT_PLAYER_LEFT | INPUT_PLAYER_RIGHT => true,
            _ => false,
        });
    if let Some(move_key) = move_key {
        player_position.translation += CELL_LENGTH
            * match *move_key {
                INPUT_PLAYER_UP => Vec3::Y,
                INPUT_PLAYER_DOWN => Vec3::NEG_Y,
                INPUT_PLAYER_LEFT => Vec3::NEG_X,
                INPUT_PLAYER_RIGHT => Vec3::X,
                _ => unreachable!(),
            };
        if let Some(start) = start_time.0 {
            let action_time = time.elapsed_seconds() - start;
            player.actions.push(Action {
                action_type: ActionType::Move(MoveDirection::Down),
                timestamp_seconds: action_time,
            }); // TODO
        } else {
            start_time.0 = Some(time.elapsed_seconds());
        }
    }
}
