use crate::math::vec2i::Vec2i;
use bevy::prelude::Resource;

#[derive(Resource, Debug, Default)]
pub struct LevelInformations {
    pub player_start_position: Vec2i,
}
