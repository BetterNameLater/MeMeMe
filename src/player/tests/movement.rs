use crate::constantes::*;
use crate::map::{Map, ObjectMap};
use crate::{
    items::events::{OnEnterEvent, OnExitEvent},
    level::ressources::level_informations::LevelInformations,
    math::vec2i::Vec2i,
    player::{
        components::player::Player, systems::player_input_system::player_move_input_system,
        GhostActions,
    },
};
use bevy::{prelude::*, time::TimePlugin};

fn init() -> App {
    let mut app = App::new();

    app.add_plugins(TimePlugin)
        .insert_resource(ButtonInput::<KeyCode>::default())
        .insert_resource(GhostActions::default())
        .insert_resource(LevelInformations::default())
        .add_event::<OnEnterEvent>()
        .add_event::<OnExitEvent>()
        .add_systems(Update, player_move_input_system);

    //spawn player and map
    app.world_mut()
        .commands()
        .spawn((Map::default(), ObjectMap));
    Player::spawn_player(&mut app.world_mut().commands(), Vec2i::default());
    app.update();

    assert_eq!(get_player(&mut app).actions.len(), 0);
    assert_eq!(
        get_player_pos(&mut app),
        &Vec2i::default().to_transform(PLAYER_Z)
    );
    app
}

fn get_player(app: &mut App) -> &Player {
    app.world_mut().query::<&Player>().single(app.world())
}

fn get_player_pos(app: &mut App) -> &Transform {
    app.world_mut()
        .query_filtered::<&Transform, With<Player>>()
        .single(app.world())
}

fn move_from_key(app: &mut App, key: KeyCode) {
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(key);
    app.update();
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .clear();
}

#[test]
fn move_up() {
    let mut app = init();

    move_from_key(&mut app, KeyCode::ArrowUp);

    assert_eq!(
        get_player_pos(&mut app),
        &Vec2i::new(0, CELL_LENGTH as i32).to_transform(PLAYER_Z)
    );
    assert_eq!(get_player(&mut app).actions.len(), 1);
}

#[test]
fn move_twice() {
    let mut app = init();

    move_from_key(&mut app, KeyCode::ArrowUp);
    move_from_key(&mut app, KeyCode::ArrowRight);

    assert_eq!(
        get_player_pos(&mut app),
        &Vec2i::new(CELL_LENGTH as i32, CELL_LENGTH as i32).to_transform(PLAYER_Z)
    );
    assert_eq!(get_player(&mut app).actions.len(), 2);
}

#[test]
fn move_back_to_start() {
    let mut app = init();

    move_from_key(&mut app, KeyCode::ArrowUp);
    move_from_key(&mut app, KeyCode::ArrowRight);
    move_from_key(&mut app, KeyCode::ArrowDown);
    move_from_key(&mut app, KeyCode::ArrowLeft);

    assert_eq!(
        get_player_pos(&mut app),
        &Vec2i::default().to_transform(PLAYER_Z)
    );
    assert_eq!(get_player(&mut app).actions.len(), 4);
}
