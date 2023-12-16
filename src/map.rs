use bevy::{ prelude::*, utils::HashMap};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, create_map)
            .add_event::<CellSpawned>();
    }
}

fn create_map(mut commands: Commands) {
    commands.spawn(Map::new(50.));
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Vec2i {
    x: i32,
    y: i32,
}
impl Vec2i {
    pub fn from(x: i32, y: i32) -> Vec2i {
        Vec2i {
            x: x,
            y: y,
        }
    }
}

#[derive(Event)]
pub struct CellSpawned(pub Entity);

#[derive(Component)]
pub struct Cell;

pub struct CellBundle {
    id: Entity,
}

#[derive(Component)]
pub struct Map {
    cells: HashMap<Vec2i, CellBundle>,
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

    pub fn map_to_local(& self, pos: Vec2i) -> Vec2 {
        Vec2 {
            x: self.cell_length * pos.x as f32,
            y: self.cell_length * pos.y as f32,
        }
    }

    pub fn spawn_cell(& mut self, pos: Vec2i, id: Entity) {
        self.cells.insert(pos, CellBundle {
            id: id,
        });
    }
}