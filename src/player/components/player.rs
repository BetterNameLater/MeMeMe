use crate::constantes::{CELL_LENGTH, PLAYER_START_TRANSFORM};
use crate::player::actions::Action;
use bevy::math::Vec2;
use bevy::prelude::{
    default, BuildChildren, Color, Commands, Component, Entity, Sprite, SpriteBundle,
};

#[derive(Component, Default, Debug)]
pub struct Player {
    pub actions: Vec<Action>,
}

impl Player {
    pub fn create_player(commands: &mut Commands, level_tag: Entity) {
        let size = CELL_LENGTH / 2.;
        commands.entity(level_tag).with_children(|parent| {
            parent.spawn((
                Player::default(),
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::BEIGE,
                        custom_size: Some(Vec2::new(size, size)),
                        ..default()
                    },
                    transform: PLAYER_START_TRANSFORM,
                    ..default()
                },
            ));
        });
    }
}
