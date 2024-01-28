use crate::constantes::*;
use crate::level::ressources::level_informations::LevelInformations;
use crate::map::{Map, ObjectMap};
use crate::math::vec2i::Vec2i;
use crate::player::actions::{Action, ActionType};
use crate::player::events::interact_event::InteractEvent;
use crate::player::events::new_position_event::NewPositionEvent;
use crate::player::{Ghost, GhostNewPositionEvent};
use bevy::ecs::query::With;
use bevy::ecs::system::Query;
use bevy::prelude::{EventWriter, Reflect, Res, ResMut, Resource};
use bevy::transform::components::Transform;

#[derive(Resource, Debug, Default, Reflect)]
pub struct GhostActions {
    pub actions: Vec<Action>,
    pub index: usize,
}

impl GhostActions {
    pub fn reset(&mut self) {
        self.actions.clear();
        self.index = 0;
    }
}

pub fn ghost_actions_system(
    mut ghost_actions: ResMut<GhostActions>,
    mut ghosts_query: Query<&mut Transform, With<Ghost>>,
    level_informations: Res<LevelInformations>,
    mut ghost_new_position_event: EventWriter<GhostNewPositionEvent>,
    mut ghost_interact_event: EventWriter<InteractEvent<Ghost>>,
    object_map_query: Query<&Map, With<ObjectMap>>,
) {
    if let Some(current_time) = level_informations.elapsed_time_from_start_rewind {
        loop {
            if ghost_actions.index >= ghost_actions.actions.len() {
                return;
            }
            let Action {
                ghost_entity: ghost_id,
                timestamp_seconds: action_time,
                action_type,
            } = &ghost_actions.actions[ghost_actions.index];
            if action_time > &current_time {
                return;
            }
            let mut ghost_transform = ghosts_query.get_mut(*ghost_id).unwrap();
            match action_type {
                ActionType::Move(move_direction) => {
                    let direction = move_direction.to_vec3();
                    let before: Vec2i = ghost_transform.translation.into();
                    ghost_transform.translation += direction * CELL_LENGTH;
                    ghost_new_position_event.send(GhostNewPositionEvent::new(
                        before,
                        ghost_transform.translation.into(),
                        *ghost_id,
                    ));
                }
                ActionType::Interact => {
                    let object_map = object_map_query.single();
                    let pos: Vec2i = ghost_transform.translation.into();
                    if let Some(item) = object_map.cells.get(&pos) {
                        ghost_interact_event.send(InteractEvent::new(
                            ghost_transform.translation.into(),
                            *ghost_id,
                            *item,
                        ));
                    }
                }
            }
            ghost_actions.index += 1;
        }
    }
}
