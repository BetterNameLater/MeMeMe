use crate::constantes::{CELL_LENGTH, PLAYER_START_TRANSFORM};
use crate::items::ghost_only::GhostOnly;
use crate::items::is_on::IsOn;
use crate::items::people_on::PeopleOn;
use crate::items::player_only::PlayerOnly;
use crate::math::vec2i::Vec2i;
use bevy::prelude::*;

// #[derive(Component)]
// pub struct PressurePlate {
//     pub level: usize,
// }
//

pub fn spawn_pressure_plate(mut commands: Commands) {
    let size = CELL_LENGTH / 3.;
    commands.spawn((
        PeopleOn(0),
        IsOn(false),
        // GhostOnly,
        SpriteBundle {
            sprite: Sprite {
                color: Color::LIME_GREEN,
                custom_size: Some(Vec2::new(size, size)),
                ..default()
            },
            // TODO in a parameter
            transform: Vec2i::new(32, 32).to_initial_map_pos(1),
            ..default()
        },
    ));
}
