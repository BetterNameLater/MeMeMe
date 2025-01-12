use super::level_state::LevelState;
use super::load_level::load_level;
use super::systems::elapsed_time_from_start_rewind_system::elapsed_time_from_start_rewind_system;
use super::systems::transitions::enter_rewind;
use super::unload_level::unload_level;
use crate::game_state::GameState;
use crate::items::events::{OnEnterEvent, OnExitEvent};
use crate::items::plugin::ItemsPlugin;
use crate::log_transitions;
use crate::player::plugin::PlayerPlugin;
use bevy::prelude::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app
            // states
            .add_sub_state::<LevelState>()
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
                (elapsed_time_from_start_rewind_system).run_if(in_state(LevelState::Playing)),
            );
        self.register_transition(app);

        #[cfg(debug_assertions)]
        app.add_systems(Update, log_transitions::<LevelState>);
    }
}

impl LevelPlugin {
    fn register_transition(&self, app: &mut App) {
        app.add_systems(OnEnter(LevelState::Rewind), enter_rewind);
    }
}
