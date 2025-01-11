use super::utils::*;
use crate::player::{
    actions::{Action, ActionType},
    components::player::Player,
    events::rewind_event::RewindEvent,
    move_direction::MoveDirection,
    GhostActions,
};
use bevy::prelude::*;

fn init() -> App {
    use crate::level::systems::elapsed_time_from_start_rewind_system::elapsed_time_from_start_rewind_system;
    use crate::player::{
        components::player::Player,
        events::{interact_event::InteractEvent, rewind_event::RewindEvent},
        systems, Ghost,
    };
    use bevy::app::Update;

    let mut app = base_init();
    app.add_systems(
        Update,
        (
            systems::player_input_system::player_move_input_system,
            systems::player_input_system::player_action_input_system,
            systems::rewind_system::rewind_system,
            elapsed_time_from_start_rewind_system,
            systems::ghost_actions_system::ghost_actions_system,
        ),
    )
    .add_event::<RewindEvent>()
    .add_event::<InteractEvent<Player>>()
    .add_event::<InteractEvent<Ghost>>();
    app
}

fn move_from_keys(keys: &[MoveDirection], ghost_entity: Entity) -> Vec<Action> {
    keys.iter()
        .enumerate()
        .map(|(i, key)| Action {
            ghost_entity,
            timestamp_seconds: i as f32,
            action_type: ActionType::Move(*key),
        })
        .collect()
}

#[test]
fn record_actions() {
    let mut app = init();
    let keys = &[MoveDirection::Down, MoveDirection::Up];
    let ghost_entity = app
        .world_mut()
        .query_filtered::<Entity, With<Player>>()
        .single(app.world());
    keys.iter().for_each(|key| {
        press_key_and_update!(app, key.into());
        advance_by!(app, SECOND);
        app.update();
    });

    assert_eq!(player(&mut app).actions, move_from_keys(keys, ghost_entity));
}

#[test]
fn transfer_actions_when_rewind() {
    let mut app = init();
    let keys = &[MoveDirection::Down, MoveDirection::Up];
    let ghost_entity = app
        .world_mut()
        .query_filtered::<Entity, With<Player>>()
        .single(app.world());
    let actions = move_from_keys(keys, ghost_entity);
    resource_mut!(app, GhostActions).actions = actions.clone();
    press_key_and_update!(app, INPUT_PLAYER_REWIND);
    app.update();

    assert_eq!(
        resource!(app, GhostActions),
        &GhostActions { actions, index: 0 }
    );
}

#[test]
fn process_actions() {
    let mut app = init();
    let keys = &[
        MoveDirection::Up,
        MoveDirection::Right,
        MoveDirection::Down,
        MoveDirection::Right,
    ];
    let ghost_entity = app
        .world_mut()
        .query_filtered::<Entity, With<Player>>()
        .single(app.world());
    // setup actions
    query_single_mut!(app, Player).actions = move_from_keys(keys, ghost_entity);
    advance_to!(app, Duration::from_secs(10));
    *resource_mut!(app, LevelInformations) = LevelInformations {
        elapsed_time_from_start_rewind: Some(10.),
        start_time: Some(0.),
        ..LevelInformations::default()
    };
    app.world_mut().send_event(RewindEvent);
    app.update();
    // start
    *resource_mut!(app, LevelInformations) = LevelInformations {
        elapsed_time_from_start_rewind: Some(0.),
        start_time: Some(15.),
        ..LevelInformations::default()
    };

    app.update();
    assert_eq!(resource!(app, GhostActions).index, 0);
    advance_to!(app, Duration::from_secs(15));
    app.update();
    assert_eq!(resource!(app, GhostActions).index, 1);
    advance_to!(app, Duration::from_secs(20));
    app.update();
    assert_eq!(resource!(app, GhostActions).index, 4);
}
