use crate::constantes::*;
use crate::map_parser::BackgroundType;
use crate::math::vec2i::Vec2i;
use bevy::color::palettes::css;
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Component)]
pub struct Cell;

#[derive(Component, Default)]
pub struct Map {
    pub cells: HashMap<Vec2i, Entity>,
}

#[derive(Component)]
pub struct WorldMap;

#[derive(Component)]
pub struct ObjectMap;

impl Map {
    fn map_to_local(&self, pos: Vec2i) -> Vec2 {
        Vec2 {
            x: CELL_LENGTH * pos.x as f32,
            y: CELL_LENGTH * pos.y as f32,
        }
    }

    pub fn spawn_cell(
        &mut self,
        commands: &mut Commands,
        parent: Entity,
        pos: Vec2i,
        background_type: &BackgroundType,
    ) {
        let cell_pos = self.map_to_local(Vec2i::new(pos.x, pos.y));
        commands.entity(parent).with_children(|parent| {
            let id = parent
                .spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: match background_type {
                                BackgroundType::Floor => css::BLUE.into(),
                                BackgroundType::Wall => css::BLACK.into(),
                                BackgroundType::Start => css::ALICE_BLUE.into(),
                                BackgroundType::End => css::GREEN.into(),
                            },
                            custom_size: Some(Vec2::new(
                                CELL_LENGTH - CELL_GAP,
                                CELL_LENGTH - CELL_GAP,
                            )),
                            ..default()
                        },
                        transform: Transform::from_translation(Vec3::new(
                            cell_pos.x, cell_pos.y, CELL_Z,
                        )),
                        ..default()
                    },
                    Cell,
                ))
                .id();
            self.cells.insert(pos, id);
        });
    }
}
