use crate::items::components::ghost_only::GhostOnly;
use crate::items::components::player_only::PlayerOnly;
use crate::items::events::{OnEnterEvent, OnExitEvent};
use crate::items::plugin::ItemsPlugin;
use crate::items::systems::people_enter_system::people_enter_system;
use crate::level::load_level::load_level;
use crate::level::ressources::level_informations::LevelInformations;
use crate::level::systems::elapsed_time_from_start_rewind_system::elapsed_time_from_start_rewind_system;
use crate::level::unload_level::unload_level;
use crate::player::plugin::PlayerPlugin;
use crate::player::systems::player_input_system::player_input_system;
use crate::player::{GhostActions, GhostNewPositionEvent, PlayerNewPositionEvent};
use crate::state::GameState;
use bevy::prelude::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app
            // resources
            .insert_resource(GhostActions::default())
            .insert_resource(LevelInformations::default())
            // plugins
            .add_plugins(PlayerPlugin)
            .add_plugins(ItemsPlugin)
            // events
            .add_event::<OnEnterEvent>()
            .add_event::<OnExitEvent>()
            // systems
            .add_systems(OnExit(GameState::InLevel), unload_level)
            .add_systems(OnEnter(GameState::LoadingLevel), load_level)
            .add_systems(
                Update,
                (
                    elapsed_time_from_start_rewind_system,
                    (
                        people_enter_system::<PlayerOnly, GhostNewPositionEvent>,
                        people_enter_system::<GhostOnly, PlayerNewPositionEvent>,
                    )
                        .after(player_input_system),
                )
                    .run_if(in_state(GameState::InLevel)),
            );
    }
}
