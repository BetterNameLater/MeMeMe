use crate::items::populate_items::populate_items;
use crate::level::components::level_tag::LevelTag;
use crate::level::components::level_to_go::LevelToGo;
use crate::map::{Map, ObjectMap, WorldMap};
use crate::map_parser::MapRepr;
use crate::math::vec2i::Vec2i;
use crate::player::player::Player;
use crate::state::GameState;
use crate::LevelAssets;
use bevy::asset::Assets;
use bevy::prelude::*;

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
                return format!("levels/{}.json", level_to_go.0 .0) == path.to_string();
            }
            false
        })
        .unwrap();

    let level = custom_assets.get(level_asset.clone()).unwrap();
    let mut world_map = Map::default();
    level
        .map
        .iter()
        .rev()
        .enumerate()
        .for_each(|(y, map_slice)| {
            map_slice
                .iter()
                .enumerate()
                .for_each(|(x, background_type)| {
                    world_map.spawn_cell(
                        &mut commands,
                        Vec2i {
                            x: x as i32,
                            y: y as i32,
                        },
                        background_type,
                    )
                })
        });
    let items = populate_items(&mut commands, &level.objects);
    let level_tag = commands.spawn((LevelTag, SpriteBundle::default())).id();
    commands.entity(level_tag).with_children(|parent| {
        parent.spawn((world_map, WorldMap));
        parent.spawn((Map { cells: items }, ObjectMap));
    });
    Player::create_player(&mut commands, level_tag);
    next_state.set(GameState::InLevel);
    commands.entity(level_to_go.1).despawn();
}
