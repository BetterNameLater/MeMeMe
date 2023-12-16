use super::math::vec2i::Vec2i;
use crate::constantes::*;
use crate::ghost::Ghost;
use crate::ghost_actions::{Action, ActionType, GhostActions, MoveDirection};
use crate::map::Map;
use crate::{ElapsedTimeFromStartRewind, StartTime};
use bevy::transform::commands;
use bevy::{prelude::*, utils::HashMap};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_player_system)
            .add_systems(Update, player_control_system)
            .add_event::<RewindEvent>();
    }
}

fn create_player(commands: &mut Commands) {
    let size = CELL_LENGTH / 2.;
    commands.spawn((
        Player::default(),
        SpriteBundle {
            sprite: Sprite {
                color: Color::BEIGE,
                custom_size: Some(Vec2::new(size, size)),
                ..default()
            },
            transform: PLAYER_START_TRANSFORM,
            ..default()
        },
    ));
}

fn create_player_system(mut commands: Commands) {
    create_player(&mut commands);
}

#[derive(Component, Default)]
pub struct Player {
    actions: Vec<Action>,
}

#[derive(Event)]
pub struct RewindEvent;

pub fn on_player_rewind_system(
    mut commands: Commands,
    mut player_query: Query<&mut Player>,
    player_entity_query: Query<Entity, With<Player>>,
    mut start_time: ResMut<StartTime>,
    mut ghost_actions: ResMut<GhostActions>,
    mut elapsed_time_from_start_rewind: ResMut<ElapsedTimeFromStartRewind>,
    mut rewind_event: EventReader<RewindEvent>,
    mut player_transform_query: Query<&mut Transform, With<Player>>,
    mut ghost_transform_query: Query<&mut Transform, (With<Ghost>, Without<Player>)>,
) {
    for ev in rewind_event.read() {
        let mut player = player_query.single_mut();
        ghost_actions.list.append(&mut player.actions);
        ghost_actions.list.sort_by(|a, b| {
            a.timestamp_seconds
                .partial_cmp(&b.timestamp_seconds)
                .unwrap()
        });
        ghost_actions.index = 0;
        player.actions.clear();

        start_time.0 = None;
        elapsed_time_from_start_rewind.0 = None;

        // reset the position of the current player, before turning im to a ghost
        *player_transform_query.single_mut() = PLAYER_START_TRANSFORM;
        commands
            .entity(player_entity_query.single())
            .remove::<Player>()
            .insert(Ghost);

        // insert a new player to replace
        create_player(&mut commands);

        // reset ghost position
        ghost_transform_query.for_each_mut(|mut ghost_transform| {
            *ghost_transform = PLAYER_START_TRANSFORM;
        });
    }
}

fn player_control_system(
    mut commands: Commands,
    mut player_transform_query: Query<&mut Transform, With<Player>>,
    /*     mut player_sprite_query: Query<&mut Sprite, With<Player>>, */
    mut player_query: Query<&mut Player>,
    player_entity_query: Query<Entity, With<Player>>,
    key_inputs: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut start_time: ResMut<StartTime>,
    mut ghost_actions: ResMut<GhostActions>,
    mut elapsed_time_from_start_rewind: ResMut<ElapsedTimeFromStartRewind>,
    mut ev_rewind: EventWriter<RewindEvent>,
) {
    let mut player_transform = player_transform_query.single_mut();
    let mut player = player_query.single_mut();
    let mut movement = Vec3::ZERO;

    let move_key = key_inputs
        .get_just_pressed()
        .find(|key_code| match **key_code {
            INPUT_PLAYER_DOWN | INPUT_PLAYER_UP | INPUT_PLAYER_LEFT | INPUT_PLAYER_RIGHT => true,
            _ => false,
        });
    if let Some(move_key) = move_key {
        let move_direction = MoveDirection::from_key_code(*move_key);
        player_transform.translation += CELL_LENGTH * move_direction.to_vec3();
        if let Some(start) = start_time.0 {
            player.actions.push(Action {
                ghost_id: player_entity_query.single(),
                action_type: ActionType::Move(move_direction),
                timestamp_seconds: elapsed_time_from_start_rewind.0.unwrap(),
            });
        } else {
            start_time.0 = Some(time.elapsed_seconds());
            elapsed_time_from_start_rewind.0 = Some(0.);
        }
        return;
    }

    // other actions
    let action_key = key_inputs
        .get_just_pressed()
        .find(|key_code| match **key_code {
            INPUT_PLAYER_REWIND => true,
            _ => false,
        });

    if let Some(action_key) = action_key {
        match action_key {
            &INPUT_PLAYER_REWIND => {
                ev_rewind.send(RewindEvent);
                println!("send RewindEvent")
            }
            _ => unreachable!(),
        }
    }
}
