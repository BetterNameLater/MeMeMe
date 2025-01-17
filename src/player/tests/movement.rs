use super::utils::*;

fn init() -> bevy::prelude::App {
    let mut app = base_init();
    app.add_systems(
        bevy::app::Update,
        crate::player::systems::player_input_system::player_move_input_system,
    );
    app
}

#[test]
fn move_up() {
    let mut app = init();

    press_key_and_update!(app, INPUT_PLAYER_UP);

    assert_eq!(
        player_transform(&mut app),
        &Vec2i::new(0, CELL_LENGTH as i32).to_transform(PLAYER_Z)
    );
    assert_eq!(player(&mut app).actions.len(), 1);
}

#[test]
fn move_twice() {
    let mut app = init();

    press_key_and_update!(app, INPUT_PLAYER_UP);
    press_key_and_update!(app, INPUT_PLAYER_RIGHT);

    assert_eq!(
        player_transform(&mut app),
        &Vec2i::new(CELL_LENGTH as i32, CELL_LENGTH as i32).to_transform(PLAYER_Z)
    );
    assert_eq!(player(&mut app).actions.len(), 2);
}

#[test]
fn move_back_to_start() {
    let mut app = init();

    press_key_and_update!(app, INPUT_PLAYER_UP);
    press_key_and_update!(app, INPUT_PLAYER_RIGHT);
    press_key_and_update!(app, INPUT_PLAYER_DOWN);
    press_key_and_update!(app, INPUT_PLAYER_LEFT);

    assert_eq!(player_transform(&mut app), &PLAYER_ORIGIN);
    assert_eq!(player(&mut app).actions.len(), 4);
}

#[test]
fn moving_starts_level_time() {
    let mut app = init();
    advance_to!(app, Duration::from_secs(6));
    press_key_and_update!(app, INPUT_PLAYER_UP);

    assert_eq!(resource!(app, PlayingTime), &PlayingTime(6., 0.))
}
