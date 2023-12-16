use super::actions::{Action, ActionType};
use super::events::RewindEvent;
use super::move_direction::MoveDirection;
use super::GhostActions;
use crate::constantes::*;
use crate::player::ghost::Ghost;
use crate::{ElapsedTimeFromStartRewind, StartTime};
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Player {
    actions: Vec<Action>,
}

impl Player {
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
}

// systems

fn create_player_system(mut commands: Commands) {
    Player::create_player(&mut commands);
}

#[allow(clippy::too_many_arguments)]
fn on_player_rewind_system(
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
    for _ in rewind_event.read() {
        let mut player = player_query.single_mut();
        ghost_actions.actions.append(&mut player.actions);
        ghost_actions.actions.sort_by(|a, b| {
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
        Player::create_player(&mut commands);

        // reset ghost position
        ghost_transform_query.for_each_mut(|mut ghost_transform| {
            *ghost_transform = PLAYER_START_TRANSFORM;
        });
    }
}

#[allow(clippy::too_many_arguments)]
fn player_input_system(
    mut player_transform_query: Query<&mut Transform, With<Player>>,
    mut player_query: Query<&mut Player>,
    player_entity_query: Query<Entity, With<Player>>,
    key_inputs: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut start_time: ResMut<StartTime>,
    mut elapsed_time_from_start_rewind: ResMut<ElapsedTimeFromStartRewind>,
    mut rewind_event: EventWriter<RewindEvent>,
) {
    // move actions
    let move_key = key_inputs.get_just_pressed().find(|key_code| {
        matches!(
            **key_code,
            INPUT_PLAYER_DOWN | INPUT_PLAYER_UP | INPUT_PLAYER_LEFT | INPUT_PLAYER_RIGHT
        )
    });
    if let Some(move_key) = move_key {
        let move_direction = MoveDirection::from_key_code(*move_key);
        player_transform_query.single_mut().translation += CELL_LENGTH * move_direction.to_vec3();
        player_query.single_mut().actions.push(Action {
            ghost_entity: player_entity_query.single(),
            action_type: ActionType::Move(move_direction),
            timestamp_seconds: elapsed_time_from_start_rewind.0.unwrap_or(0.),
        });
        if elapsed_time_from_start_rewind.0.is_none() {
            start_time.0 = Some(time.elapsed_seconds());
            elapsed_time_from_start_rewind.0 = Some(0.);
        }
        return;
    }

    // other actions
    let action_key = key_inputs
        .get_just_pressed()
        .find(|key_code| matches!(**key_code, INPUT_PLAYER_REWIND));
    if let Some(action_key) = action_key {
        match action_key {
            &INPUT_PLAYER_REWIND => {
                rewind_event.send(RewindEvent);
            }
            _ => unreachable!(),
        }
    }
}

// plugins

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_player_system)
            .add_systems(Update, (player_input_system, on_player_rewind_system))
            .add_event::<RewindEvent>();
    }
}
