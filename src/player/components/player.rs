use super::person::Person;
use crate::constantes::{CELL_LENGTH, PLAYER_Z};
use crate::math::vec2i::Vec2i;
use crate::player::actions::Action;
use bevy::core::Name;
use bevy::math::Vec2;
use bevy::prelude::{default, Commands, Component, Entity, Sprite};

#[derive(Component, Default, Debug)]
pub struct Player {
    pub actions: Vec<Action>,
}

impl Person for Player {}

impl Player {
    pub fn spawn_player(commands: &mut Commands, position: Vec2i) -> Entity {
        let size = CELL_LENGTH / 2.;

        commands
            .spawn((
                Name::new("Player"),
                Player::default(),
                Sprite {
                    color: bevy::color::palettes::css::BEIGE.into(),
                    custom_size: Some(Vec2::new(size, size)),
                    ..default()
                },
                position.to_transform(PLAYER_Z),
            ))
            .id()
    }
}
