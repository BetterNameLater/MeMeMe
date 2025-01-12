use super::ressources::level_informations::{GhostCount, StartPosition};
use crate::constantes::*;
use crate::game_state::GameState;
use crate::items::populate_items::populate_items;
use crate::items::primitive::colliding::Colliding;
use crate::level::components::level_tag::LevelTag;
use crate::level::components::level_to_go::LevelToGo;
use crate::level::ressources::level_informations::LevelInformations;
use crate::map::{ObjectMap, WorldMap};
use crate::map_parser::{BackgroundType, MapRepr};
use crate::math::vec2i::Vec2i;
use crate::player::components::player::Player;
use crate::player::GhostActions;
use crate::LevelAssets;
use bevy::asset::Assets;
use bevy::color::palettes::css;
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
                spawn_cell_into_parent(
                    &mut commands,
                    world_map_entity,
                    Vec2i {
                        x: x as i32,
                        y: y as i32,
                    },
                    background_type,
                );
            })
    });

    commands.entity(level_tag).add_child(world_map_entity);

    let start_position = find_start_position(&map);

    let items_map_entity = commands
        .spawn((ObjectMap(items), Name::new("ObjectMap")))
        .id();
    commands.entity(level_tag).add_child(items_map_entity);
    let player = Player::spawn_player(&mut commands, start_position);
    commands.entity(level_tag).add_child(player);

    next_state.set(GameState::InLevel);
    commands.entity(level_to_go.1).despawn();

    // insert resources
    commands.insert_resource(GhostActions::default());
    commands.insert_resource(GhostCount(0));
    commands.insert_resource(LevelInformations::default());
    commands.insert_resource(StartPosition::new(start_position));
}

fn map_to_local(pos: Vec2i) -> Vec2 {
    Vec2 {
        x: CELL_LENGTH * pos.x as f32,
        y: CELL_LENGTH * pos.y as f32,
    }
}

fn spawn_cell_into_parent(
    commands: &mut Commands,
    parent: Entity,
    pos: Vec2i,
    background_type: &BackgroundType,
) {
    let Vec2 { x, y } = map_to_local(Vec2i::new(pos.x, pos.y));

    commands.entity(parent).with_children(|parent| {
        let _ = parent
            .spawn((
                Sprite {
                    color: match background_type {
                        BackgroundType::Floor => css::BLUE.into(),
                        BackgroundType::Wall => css::BLACK.into(),
                        BackgroundType::Start => css::ALICE_BLUE.into(),
                        BackgroundType::End => css::GREEN.into(),
                        BackgroundType::Void => unreachable!("should not try spawning void"),
                    },
                    custom_size: Some(Vec2::new(CELL_LENGTH - CELL_GAP, CELL_LENGTH - CELL_GAP)),
                    ..default()
                },
                Transform::from_translation(Vec3::new(x, y, CELL_Z)),
            ))
            .insert_if(Colliding, || {
                matches!(background_type, BackgroundType::Wall)
            });
    });
}
