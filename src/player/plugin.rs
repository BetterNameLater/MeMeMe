use super::{ghost_actions_system, Ghost};
use crate::game_state::GameState;
use crate::level::level_state::LevelState;
use crate::player::components::player::Player;
use crate::player::events::interact_event::InteractEvent;
use crate::player::systems::player_input_system::*;
use crate::player::systems::transitions::enter_rewind;
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                player_move_input_system,
                player_action_input_system,
                ghost_actions_system,
            )
                .run_if(in_state(GameState::InLevel)),
        )
        .add_event::<InteractEvent<Player>>()
        .add_event::<InteractEvent<Ghost>>();

        self.register_transitions(app);
    }
}

impl PlayerPlugin {
    fn register_transitions(&self, app: &mut App) {
        app.add_systems(OnEnter(LevelState::Rewind), enter_rewind);
    }
}
