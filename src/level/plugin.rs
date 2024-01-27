use crate::items::components::ghost_only::GhostOnly;
use crate::items::components::player_only::PlayerOnly;
use crate::items::events::{OnEnterEvent, OnExitEvent};
use crate::items::systems::count_people_on_system::count_people_on_system;
use crate::items::systems::level_teleporter_system::level_teleporter_system;
use crate::items::systems::people_enter_system::people_enter_system;
use crate::items::systems::teleporter_system::{teleporter_activate_system, teleporter_system};
use crate::items::systems::toggle_on_system::toggle_on_enter_system;
use crate::items::systems::update_is_activated_system::update_is_activated_system;
use crate::level::load_level::load_level;
use crate::level::unload_level::unload_level;
use crate::player::components::player::Player;
use crate::player::plugin::PlayerPlugin;
use crate::player::systems::player_input_system::player_input_system;
use crate::player::{Ghost, GhostActions, GhostNewPositionEvent, PlayerNewPositionEvent};
use crate::state::GameState;
use crate::time::{elapsed_time_from_start_rewind_system, ElapsedTimeFromStartRewind, StartTime};
use bevy::prelude::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app
            // resources
            .insert_resource(GhostActions::default())
            .insert_resource(StartTime(None))
            .insert_resource(ElapsedTimeFromStartRewind(None))
            // plugins
            .add_plugins(PlayerPlugin)
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
                    (
                        level_teleporter_system,
                        count_people_on_system::<GhostOnly, Player>,
                        count_people_on_system::<PlayerOnly, Ghost>,
                        teleporter_system::<PlayerOnly, Ghost>,
                        teleporter_system::<GhostOnly, Player>,
                        teleporter_activate_system::<PlayerOnly, Ghost>,
                        teleporter_activate_system::<GhostOnly, Player>,
                        toggle_on_enter_system::<PlayerOnly, Ghost>,
                        toggle_on_enter_system::<GhostOnly, Player>,
                    )
                        .after(people_enter_system::<PlayerOnly, GhostNewPositionEvent>)
                        .after(people_enter_system::<GhostOnly, PlayerNewPositionEvent>),
                    update_is_activated_system,
                )
                    .run_if(in_state(GameState::InLevel)),
            );
    }
}
