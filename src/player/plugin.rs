use super::systems::input_system::{
    idle_move_input_system, interact_input_system, is_move_input_pressed, move_input_system,
};
use super::{actions_system, Ghost};
use crate::constantes::*;
use crate::items::interaction_type::ghost_only::GhostOnly;
use crate::items::interaction_type::player_only::PlayerOnly;
use crate::level::level_state::LevelState;
use crate::player::components::player::Player;
use crate::player::events::interact_event::InteractEvent;
use crate::player::systems::transitions::{enter_playing, enter_rewind};
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use bevy::utils::tracing::field::debug;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                move_input_system.run_if(in_state(LevelState::Playing)),
                idle_move_input_system.run_if(in_state(LevelState::Idle)),
            )
                .run_if(is_move_input_pressed),
        )
        .add_systems(
            Update,
            (
                actions_system::<Ghost, PlayerOnly>,
                actions_system::<Player, GhostOnly>,
            )
                .run_if(in_state(LevelState::Playing))
                .after(move_input_system),
        )
        .add_systems(
            Update,
            (
                (|mut next_state: ResMut<NextState<LevelState>>| {
                    debug("rewind");
                    next_state.set(LevelState::Rewind);
                })
                .run_if(input_just_pressed(input::REWIND)),
                interact_input_system.run_if(input_just_pressed(input::INTERACT)),
            )
                .run_if(in_state(LevelState::Playing)),
        )
        .add_event::<InteractEvent<Player>>()
        .add_event::<InteractEvent<Ghost>>();

        self.register_transitions(app);
    }
}

impl PlayerPlugin {
    fn register_transitions(&self, app: &mut App) {
        app.add_systems(OnEnter(LevelState::Rewind), enter_rewind);
        app.add_systems(OnEnter(LevelState::Playing), enter_playing);
    }
}
