use crate::items::components::ghost_only::GhostOnly;
use crate::items::components::player_only::PlayerOnly;
use crate::items::events::OnEnterEvent;
use crate::items::systems::count_people_on_system::count_people_on_system;
use crate::items::systems::level_teleporter_system::level_teleporter_system;
use crate::items::systems::people_enter_system::people_enter_system;
use crate::items::systems::teleporter_system::{teleporter_activate_system, teleporter_system};
use crate::items::systems::toggle::{toggle_on_enter_system, toggle_on_interact_system};
use crate::items::systems::update_is_activated_system::update_is_activated_system;
use crate::level::load_level::load_level;
use crate::level::unload_level::unload_level;
use crate::player::interact::{GhostInteractEvent, PlayerInteractEvent};
use crate::player::player::{player_input_system, Player, PlayerPlugin};
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
                        count_people_on_system::<GhostOnly, PlayerNewPositionEvent>,
                        count_people_on_system::<PlayerOnly, GhostNewPositionEvent>,
                        teleporter_system::<PlayerOnly, GhostNewPositionEvent, Ghost>,
                        teleporter_system::<GhostOnly, PlayerNewPositionEvent, Player>,
                        teleporter_activate_system::<PlayerOnly, Ghost>,
                        teleporter_activate_system::<GhostOnly, Player>,
                        toggle_on_interact_system::<GhostOnly, PlayerInteractEvent>,
                        toggle_on_interact_system::<PlayerOnly, GhostInteractEvent>,
                        toggle_on_enter_system::<PlayerOnly, GhostNewPositionEvent>,
                        toggle_on_enter_system::<GhostOnly, PlayerNewPositionEvent>,
                    )
                        .after(people_enter_system::<PlayerOnly, GhostNewPositionEvent>)
                        .after(people_enter_system::<GhostOnly, PlayerNewPositionEvent>),
                    update_is_activated_system,
                )
                    .run_if(in_state(GameState::InLevel)),
            );
    }
}
