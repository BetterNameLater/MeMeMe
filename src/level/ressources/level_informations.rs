use crate::math::vec2i::Vec2i;
use bevy::prelude::*;

/// Immutable level informations
#[derive(Reflect, Resource, Debug, PartialEq)]
pub struct StartPosition(Vec2i);

impl StartPosition {
    pub fn new(player_start_position: Vec2i) -> Self {
        Self(player_start_position)
    }

    pub fn get(&self) -> &Vec2i {
        &self.0
    }
}

/// The quantity of ghost spawned. It also reflect the number of time the rewind action was done in a row
#[derive(Reflect, Resource, Debug, PartialEq)]
pub struct GhostCount(pub usize);

/// Time information for the current run. Only available in LevelState::Playing
///
/// Tuple (start_time, elapsed_time_from_start_rewind)
#[derive(Reflect, Resource, Debug, Default, PartialEq)]
pub struct PlayingTime(pub f32, pub f32);
