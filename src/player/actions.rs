use crate::player::move_direction::MoveDirection;
use bevy::prelude::{Entity, Reflect};

#[derive(Debug, Reflect)]
pub struct Action {
    pub ghost_entity: Entity,
    pub timestamp_seconds: f32,
    pub action_type: ActionType,
}

#[derive(Debug, Reflect)]
pub enum ActionType {
    Move(MoveDirection),
    Interact,
}
