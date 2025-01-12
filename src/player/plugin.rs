use super::{ghost_actions_system, Ghost};
use crate::game_state::GameState;
use crate::player::components::player::Player;
use crate::player::events::interact_event::InteractEvent;
use crate::player::events::rewind_event::RewindEvent;
use crate::player::systems::player_input_system::*;
use crate::player::systems::rewind_system::rewind_system;
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
                rewind_system,
            )
                .run_if(in_state(GameState::InLevel)),
        )
        .add_event::<RewindEvent>()
        .add_event::<InteractEvent<Player>>()
        .add_event::<InteractEvent<Ghost>>();
    }
}
