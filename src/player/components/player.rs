use crate::constantes::{CELL_LENGTH, PLAYER_Z};
use crate::math::vec2i::Vec2i;
use crate::player::actions::Action;
use bevy::math::Vec2;
use bevy::prelude::{default, Color, Commands, Component, Entity, Sprite, SpriteBundle};

#[derive(Component, Default, Debug)]
pub struct Player {
    pub actions: Vec<Action>,
}

impl Player {
    pub fn spawn_player(commands: &mut Commands, position: Vec2i) -> Entity {
        let size = CELL_LENGTH / 2.;

        return commands
            .spawn((
                Player::default(),
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::BEIGE,
                        custom_size: Some(Vec2::new(size, size)),
                        ..default()
                    },
                    transform: position.to_transform(PLAYER_Z as i32),
                    ..default()
                },
            ))
            .id();
    }
}
