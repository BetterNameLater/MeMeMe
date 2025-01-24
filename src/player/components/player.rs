use super::person::Person;
use crate::constantes::{CELL_LENGTH, PLAYER_Z};
use bevy::core::Name;
use bevy::prelude::*;

#[derive(Component, Default, Debug, Reflect)]
#[require(Name(||Name::new("Player")))]
pub struct Player;

impl Person for Player {}

impl Player {
    pub fn spawn_player(commands: &mut Commands, position: IVec2) -> Entity {
        let size = CELL_LENGTH / 2.;

        commands
            .spawn((
                Player,
                Sprite {
                    color: bevy::color::palettes::css::BEIGE.into(),
                    custom_size: Some(Vec2::new(size, size)),
                    ..default()
                },
                Transform::from_translation(position.as_vec2().extend(PLAYER_Z)),
            ))
            .id()
    }
}
