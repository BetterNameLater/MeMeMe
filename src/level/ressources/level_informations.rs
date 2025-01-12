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

#[derive(Reflect, Resource, Debug, Default, PartialEq)]
pub struct LevelInformations {
    pub ghost_count: usize,
    pub start_time: Option<f32>,
    pub elapsed_time_from_start_rewind: Option<f32>,
}

impl LevelInformations {
    pub fn rewind(&mut self) {
        self.ghost_count += 1;
        self.start_time = None;
        self.elapsed_time_from_start_rewind = None;
    }
}
