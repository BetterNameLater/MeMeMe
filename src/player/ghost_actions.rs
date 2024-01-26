use super::actions::{Action, ActionType};
use super::ghost::Ghost;
use crate::constantes::*;
use crate::math::vec2i::Vec2i;
use crate::player::events::new_position_event::NewPositionEvent;
use crate::player::GhostNewPositionEvent;
use crate::ElapsedTimeFromStartRewind;
use bevy::ecs::query::With;
use bevy::ecs::system::Query;
use bevy::prelude::{EventWriter, Res, ResMut, Resource};
use bevy::transform::components::Transform;

#[derive(Resource, Debug, Default)]
pub struct GhostActions {
    pub actions: Vec<Action>,
    pub index: usize,
}

// systems

pub fn ghost_actions_system(
    mut ghost_actions: ResMut<GhostActions>,
    mut ghosts_query: Query<&mut Transform, With<Ghost>>,
    elapsed_time_from_start_rewind: Res<ElapsedTimeFromStartRewind>,
    mut ghost_new_position_event: EventWriter<GhostNewPositionEvent>,
) {
    if let Some(current_time) = elapsed_time_from_start_rewind.0 {
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
            match action_type {
                ActionType::Move(move_direction) => {
                    let direction = move_direction.to_vec3();
                    let mut ghost_transform = ghosts_query.get_mut(*ghost_id).unwrap();
                    let before: Vec2i = ghost_transform.translation.into();
                    ghost_transform.translation += direction * CELL_LENGTH;
                    ghost_new_position_event.send(GhostNewPositionEvent::new(
                        before,
                        ghost_transform.translation.into(),
                        *ghost_id,
                    ));
                }
            }
            ghost_actions.index += 1;
        }
    }
}
