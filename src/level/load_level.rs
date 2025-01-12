use crate::constantes::CELL_LENGTH_USIZE;
use crate::items::populate_items::populate_items;
use crate::level::components::level_tag::LevelTag;
use crate::level::components::level_to_go::LevelToGo;
use crate::level::ressources::level_informations::LevelInformations;
use crate::map::{Map, ObjectMap, WorldMap};
use crate::map_parser::{BackgroundType, MapRepr};
use crate::math::vec2i::Vec2i;
use crate::player::components::player::Player;
use crate::player::GhostActions;
use crate::state::GameState;
use crate::LevelAssets;
use bevy::asset::Assets;
use bevy::prelude::*;

fn find_start_position(map: &[Vec<BackgroundType>]) -> Vec2i {
    for (y, row) in map.iter().rev().enumerate() {
        for (x, background_type) in row.iter().enumerate() {
            if background_type == &BackgroundType::Start {
                return Vec2i::new(
                    (x * CELL_LENGTH_USIZE) as i32,
                    (y * CELL_LENGTH_USIZE) as i32,
                );
            }
        }
    }
    Vec2i::default()
}

#[allow(clippy::too_many_arguments)]
pub fn load_level(
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
    custom_assets: Res<Assets<MapRepr>>,
    mut next_state: ResMut<NextState<GameState>>,
    level_to_go_query: Query<(&LevelToGo, Entity)>,
    mut ghost_actions: ResMut<GhostActions>,
    mut level_infos: ResMut<LevelInformations>,
) {
    let level_to_go = level_to_go_query.single();
    let level_asset = level_assets
        .levels
        .iter()
        .find(|a| {
            if let Some(path) = a.path() {
                if cfg!(target_os = "windows") {
                    let level_path = level_to_go.0 .0.to_string().replace("/", "\\");
                    return format!("levels\\{}.json", level_path) == path.to_string();
                }
                return format!("levels/{}.json", level_to_go.0 .0) == path.to_string();
            }
            false
        })
        .unwrap_or_else(|| panic!("could not find `levels/{}.json", level_to_go.0 .0));

    let level = custom_assets.get(level_asset).unwrap();

    let level_tag = commands
        .spawn((
            LevelTag,
            Sprite::default(),
            Name::new(level_asset.path().unwrap().to_string()),
        ))
        .id();

    let mut world_map = Map::default();
    let world_map_entity = commands
        .spawn((WorldMap, Sprite::default(), Name::new("WorldMap")))
        .id();

    let items = populate_items(&mut commands, level_tag, &level.objects);
    let map = level.map();
    map.iter().rev().enumerate().for_each(|(y, map_slice)| {
        map_slice
            .iter()
            .enumerate()
            .filter(|(_, t)| !matches!(t, &BackgroundType::Void))
            .for_each(|(x, background_type)| {
                world_map.spawn_cell(
                    &mut commands,
                    world_map_entity,
                    Vec2i {
                        x: x as i32,
                        y: y as i32,
                    },
                    background_type,
                )
            })
    });

    commands.entity(world_map_entity).insert(world_map);
    commands.entity(level_tag).add_child(world_map_entity);

    let start_position = find_start_position(&map);

    let items_map_entity = commands
        .spawn((Map { cells: items }, ObjectMap, Name::new("ObjectMap")))
        .id();
    commands.entity(level_tag).add_child(items_map_entity);
    let player = Player::spawn_player(&mut commands, start_position);
    commands.entity(level_tag).add_child(player);

    next_state.set(GameState::InLevel);
    commands.entity(level_to_go.1).despawn();

    // reset ressources
    ghost_actions.reset();
    level_infos.reset(start_position);
}
