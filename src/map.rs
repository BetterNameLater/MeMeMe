use crate::constantes::*;
use crate::map_parser::BackgroundType;
use crate::math::vec2i::Vec2i;
use bevy::{prelude::*, utils::HashMap};

#[derive(Event)]
pub struct CellSpawned(pub Entity);

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
    pub fn get_cell_entity_by_pos(&self, pos: &Vec2i) -> Option<Entity> {
        self.cells.get(pos).copied()
    }

    pub fn map_to_local(&self, pos: Vec2i) -> Vec2 {
        Vec2 {
            x: CELL_LENGTH * pos.x as f32,
            y: CELL_LENGTH * pos.y as f32,
        }
    }

    pub fn spawn_cell(
        &mut self,
        commands: &mut Commands,
        pos: Vec2i,
        background_type: &BackgroundType,
    ) {
        let cell_pos = self.map_to_local(Vec2i::new(pos.x, pos.y));
        let id = commands
            .spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: match background_type {
                            BackgroundType::Floor => Color::BLUE,
                            BackgroundType::Wall => Color::BLACK,
                            BackgroundType::Start => Color::ALICE_BLUE,
                            BackgroundType::End => Color::GREEN,
                        },
                        custom_size: Some(Vec2::new(
                            CELL_LENGTH - CELL_GAP,
                            CELL_LENGTH - CELL_GAP,
                        )),
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(cell_pos.x, cell_pos.y, 0.)),
                    ..default()
                },
                Cell,
            ))
            .id();
        self.cells.insert(pos, id);
    }
}
