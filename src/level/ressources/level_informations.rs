use crate::math::vec2i::Vec2i;
use bevy::prelude::*;

#[derive(Reflect, Resource, Debug, Default)]
pub struct LevelInformations {
    pub player_start_position: Vec2i,
    pub ghost_count: usize,
    pub start_time: Option<f32>,
    pub elapsed_time_from_start_rewind: Option<f32>,
}

impl LevelInformations {
    pub fn reset(&mut self, player_start_position: Vec2i) {
        self.ghost_count = 0;
        self.player_start_position = player_start_position;
        self.start_time = None;
        self.elapsed_time_from_start_rewind = None;
    }

    pub fn rewind(&mut self) {
        self.ghost_count += 1;
        self.start_time = None;
        self.elapsed_time_from_start_rewind = None;
    }
}
