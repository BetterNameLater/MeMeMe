use crate::constantes::*;
use crate::math::vec2i::Vec2i;
use bevy::{prelude::*, utils::HashMap};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, create_map)
            .add_event::<CellSpawned>();
    }
}

fn create_map(mut commands: Commands) {
    commands.spawn(Map::default());
}

#[derive(Event)]
pub struct CellSpawned(pub Entity);

#[derive(Component)]
pub struct Cell;

#[derive(Component, Default)]
pub struct Map {
    cells: HashMap<Vec2i, Entity>,
}

impl Map {
    pub fn get_cell_entity_by_pos(&self, pos: &Vec2i) -> Option<Entity> {
        self.cells.get(pos).map(|e| e.clone())
    }

    pub fn map_to_local(&self, pos: Vec2i) -> Vec2 {
        Vec2 {
            x: CELL_LENGTH * pos.x as f32,
            y: CELL_LENGTH * pos.y as f32,
        }
    }

    pub fn spawn_cell(&mut self, commands: &mut Commands, pos: Vec2i) {
        let cell_pos = self.map_to_local(Vec2i::from(pos.x, pos.y));
        let id = commands
            .spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(0.25, 0.25, 0.75),
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
