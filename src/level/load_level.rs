use super::ressources::level_informations::{GhostCount, StartPosition};
use crate::constantes::*;
use crate::game_state::GameState;
use crate::items::populate_items::populate_items;
use crate::items::primitive::colliding::Colliding;
use crate::items::primitive::is_activated::IsActivated;
use crate::items::win::Win;
use crate::level::components::level_tag::LevelTag;
use crate::level::components::level_to_go::LevelToGo;
use crate::map::WorldMap;
use crate::player::actions::ActionStack;
use crate::player::components::player::Player;
use crate::player::Ghost;
use crate::LevelAssets;
use bevy::asset::Assets;
use bevy::color::palettes::css;
use bevy::prelude::*;
use level_parser::BackgroundType;
use level_parser::WorldRepr;

#[allow(clippy::too_many_arguments)]
pub fn load_level(
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
    custom_assets: Res<Assets<WorldRepr>>,
    mut next_state: ResMut<NextState<GameState>>,
    level_to_go: Res<LevelToGo>,
) {
    let world = custom_assets.get(level_assets.world.id()).unwrap();
    let level = world.levels.get(&level_to_go.0).unwrap();

    let level_tag = commands
        .spawn((LevelTag, Name::new(level_to_go.0.clone())))
        .id();

    let world_map_entity = commands
        .spawn((WorldMap, Sprite::default(), Name::new("WorldMap")))
        .id();

    populate_items(&mut commands, level_tag, &level.objects);
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
                    IVec2 {
                        x: x as i32,
                        y: y as i32,
                    },
                    background_type,
                );
            })
    });

    commands.entity(level_tag).add_child(world_map_entity);

    let start_position = level.start * CELL_LENGTH as i32;
    let goal_position = level.goal.map(|v| v * CELL_LENGTH as i32);

    if let Some(goal_position) = goal_position {
        commands.entity(level_tag).with_child((
            Win,
            IsActivated(true),
            Transform::from_translation(goal_position.as_vec2().extend(FLAG_Z)),
            Sprite {
                color: bevy::color::palettes::css::GREEN.into(),
                custom_size: Some(Vec2::new(CELL_LENGTH - CELL_GAP, CELL_LENGTH - CELL_GAP)),
                ..default()
            },
        ));
    }

    let player = Player::spawn_player(&mut commands, start_position);
    commands.entity(level_tag).add_child(player);

    next_state.set(GameState::InLevel);
    commands.remove_resource::<LevelToGo>();

    // insert resources
    commands.init_resource::<ActionStack<Ghost>>();
    commands.init_resource::<ActionStack<Player>>();
    commands.init_resource::<GhostCount>();
    commands.insert_resource(StartPosition::new(start_position));
}

fn map_to_local(pos: IVec2) -> Vec2 {
    Vec2 {
        x: CELL_LENGTH * pos.x as f32,
        y: CELL_LENGTH * pos.y as f32,
    }
}

fn spawn_cell_into_parent(
    commands: &mut Commands,
    parent: Entity,
    pos: IVec2,
    background_type: &BackgroundType,
) {
    let Vec2 { x, y } = map_to_local(IVec2::new(pos.x, pos.y));

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
