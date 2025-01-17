use super::utils::*;

fn init() -> bevy::prelude::App {
    use crate::level::systems::elapsed_time_from_start_rewind_system::elapsed_time_from_start_rewind_system;
    use crate::player::{
        components::player::Player, events::interact_event::InteractEvent, systems, Ghost,
    };
    use bevy::app::Update;

    let mut app = base_init();
    app.add_systems(
        Update,
        (
            systems::player_input_system::player_move_input_system,
            systems::player_input_system::player_action_input_system,
            elapsed_time_from_start_rewind_system,
        ),
    )
    .add_event::<InteractEvent<Player>>()
    .add_event::<InteractEvent<Ghost>>();
    app
}

#[test]
fn it_reset_player() {
    let mut app = init();

    press_key_and_update!(app, input::UP);
    press_key_and_update!(app, input::REWIND);
    app.update();

    assert_eq!(player_transform(&mut app), &PLAYER_ORIGIN);
    assert_eq!(player(&mut app).actions.len(), 0);
}

#[test]
fn it_set_level_infos() {
    let mut app = init();
    press_key_and_update!(app, input::UP);
    press_key_and_update!(app, input::REWIND);
    app.update();

    assert_eq!(resource!(app, PlayingTime), &PlayingTime(0., 0.));

    app.update();
}
