use crate::items::enterable::people_enter_system;
use crate::items::events::OnEnterEvent;
use crate::items::ghost_only::GhostOnly;
use crate::items::level_teleporter::level_teleporter_system;
use crate::items::player_only::PlayerOnly;
use crate::items::systems::is_activated::update_is_activated_system;
use crate::items::systems::people_on::count_people_on_system;
use crate::items::systems::teleport::{teleporter_activate_system, teleporter_system};
use crate::items::systems::toggle::{toggle_on_enter_system, toggle_on_interact_system};
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
