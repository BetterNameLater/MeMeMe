use super::math::vec2i::Vec2i;
use crate::constantes::*;
use crate::ghost_actions::{Action, ActionType, GhostActions, MoveDirection};
use crate::map::Map;
use crate::StartTime;
use bevy::transform::commands;
use bevy::{prelude::*, utils::HashMap};

#[derive(Component)]
pub struct Ghost;
