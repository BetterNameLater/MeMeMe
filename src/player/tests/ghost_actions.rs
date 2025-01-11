use super::utils::*;
use crate::player::{
    actions::{Action, ActionType},
    components::player::Player,
    move_direction::MoveDirection,
    GhostActions,
};
use bevy::{
    app::App,
    prelude::{Entity, With},
};

fn init() -> bevy::prelude::App {
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
        ),
    )
    .add_event::<RewindEvent>()
    .add_event::<InteractEvent<Player>>()
    .add_event::<InteractEvent<Ghost>>();
    app
}

fn move_from_keys(app: &mut App, keys: &[MoveDirection], ghost_entity: Entity) -> Vec<Action> {
    keys.iter().for_each(|key| {
        press_key_and_update!(app, key.into());
        advance_by!(app, SECOND);
        app.update();
    });
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
fn record_action() {
    let mut app = init();
    let keys = &[MoveDirection::Down, MoveDirection::Up];
    let ghost_entity = app
        .world_mut()
        .query_filtered::<Entity, With<Player>>()
        .single(app.world());
    let actions = move_from_keys(&mut app, keys, ghost_entity);

    assert_eq!(player(&mut app).actions, actions);

    press_key_and_update!(app, INPUT_PLAYER_REWIND);
    app.update();

    assert_eq!(
        resource!(app, GhostActions),
        &GhostActions { actions, index: 0 }
    );
}
