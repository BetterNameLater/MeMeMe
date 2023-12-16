use crate::constantes::*;
use crate::player::move_direction::MoveDirection;
use bevy::math::Vec3;
use bevy::prelude::{Entity, KeyCode};

#[derive(Debug)]
pub struct Action {
    pub ghost_entity: Entity,
    pub timestamp_seconds: f32,
    pub action_type: ActionType,
}

#[derive(Debug)]
pub enum ActionType {
    Move(MoveDirection),
}
