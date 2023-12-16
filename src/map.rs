use crate::math::vec2i::Vec2i;
use bevy::{prelude::*, utils::HashMap};

pub struct MapPlugin;

const CELL_LENGTH: f32 = 32.;
const CELL_GAP: f32 = 8.;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, create_map)
            .add_event::<CellSpawned>();
    }
}

fn create_map(mut commands: Commands) {
    commands.spawn(Map::new(CELL_LENGTH));
}

#[derive(Event)]
pub struct CellSpawned(pub Entity);

#[derive(Component)]
pub struct Cell;

pub struct CellData {
    id: Entity,
}

#[derive(Component)]
pub struct Map {
    cells: HashMap<Vec2i, CellData>,
    cell_length: f32,
}

impl Map {
    pub fn new(cell_length: f32) -> Map {
        Map {
            cells: HashMap::new(),
            cell_length: cell_length,
        }
    }

    pub fn get_cell_length(&self) -> f32 {
        self.cell_length
    }

    pub fn map_to_local(&self, pos: Vec2i) -> Vec2 {
        Vec2 {
            x: self.cell_length * pos.x as f32,
            y: self.cell_length * pos.y as f32,
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
                            self.get_cell_length() - CELL_GAP,
                            self.get_cell_length() - CELL_GAP,
                        )),
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(
                        cell_pos.x,
                        cell_pos.y,
                        0.,
                    )),
                    ..default()
                },
                Cell,
            ))
            .id();
        self.cells.insert(pos, CellData { id });
    }
}
