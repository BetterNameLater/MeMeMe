use crate::math::vec2i::Vec2i;
use bevy::prelude::*;

#[derive(Reflect, Resource, Debug, Default)]
pub struct LevelInformations {
    pub player_start_position: Vec2i,
    pub ghost_count: usize,
}

impl LevelInformations {
    pub fn reset(&mut self, player_start_position: Vec2i) {
        self.ghost_count = 0;
        self.player_start_position = player_start_position;
    }
}
