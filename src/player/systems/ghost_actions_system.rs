use crate::constantes::*;
use crate::items::events::OnEnterEvent;
use crate::items::events::OnExitEvent;
use crate::items::primitive::enterable::EnterAble;
use crate::items::primitive::player_only::PlayerOnly;
use crate::level::ressources::level_informations::LevelInformations;
use crate::map::{Map, ObjectMap};
use crate::math::vec2i::Vec2i;
use crate::player::actions::{Action, ActionType};
use crate::player::events::interact_event::InteractEvent;
use crate::player::events::new_position_event::NewPositionEventData;
use crate::player::{systems::player_input_system::add_enter_exit_event, Ghost};
use bevy::prelude::*;

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

#[allow(clippy::too_many_arguments)]
pub fn ghost_actions_system(
    mut ghost_actions: ResMut<GhostActions>,
    mut ghosts_query: Query<&mut Transform, With<Ghost>>,
    level_informations: Res<LevelInformations>,
    mut ghost_interact_event: EventWriter<InteractEvent<Ghost>>,
    object_map_query: Query<&Map, With<ObjectMap>>,
    player_only_people_on_query: Query<(), (With<EnterAble>, Without<PlayerOnly>)>,
    mut on_enter_event_writer: EventWriter<OnEnterEvent>,
    mut on_exit_event_writer: EventWriter<OnExitEvent>,
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

                    let new_position_event = NewPositionEventData {
                        before,
                        now: ghost_transform.translation.into(),
                        entity: *ghost_id,
                    };
                    add_enter_exit_event(
                        new_position_event,
                        &object_map_query,
                        &player_only_people_on_query,
                        &mut on_enter_event_writer,
                        &mut on_exit_event_writer,
                    );
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
