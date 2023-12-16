use bevy::input::Input;
use bevy::prelude::{KeyCode, MouseButton, Res, Resource};

#[derive(Resource, Debug)]
pub struct GhostActions {
    pub list: Vec<Action>,
    pub index: usize,
}

pub fn actions_system(ghost_actions: Res<GhostActions>) {
    println!("{:?}", ghost_actions);
}

#[derive(Debug)]

pub struct Action {
    pub timestamp_seconds: f32,
    pub action_type: ActionType,
}

#[derive(Debug)]

pub enum ActionType {
    Move(MoveDirection),
    // Levier
    // TODO
}

#[derive(Debug)]

pub enum MoveDirection {
    Up,
    Left,
    Down,
    Right,
}
