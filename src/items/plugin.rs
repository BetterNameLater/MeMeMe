use crate::items::components::ghost_only::GhostOnly;
use crate::items::components::player_only::PlayerOnly;
use crate::items::systems::count_people_on_system::count_people_on_system;
use crate::items::systems::level_teleporter_system::level_teleporter_system;
use crate::items::systems::people_enter_system::people_enter_system;
use crate::items::systems::teleporter_system::{teleporter_activate_system, teleporter_system};
use crate::items::systems::toggle_on_system::{toggle_on_enter_system, toggle_on_interact_system};
use crate::items::systems::update_is_activated_system::{
    update_is_unusable_system, update_is_usable_system,
};
use crate::player::components::player::Player;
use crate::player::{Ghost, GhostNewPositionEvent, PlayerNewPositionEvent};
use crate::state::GameState;
use bevy::prelude::*;

pub struct ItemsPlugin;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_is_usable_system
                    .after(people_enter_system::<PlayerOnly, GhostNewPositionEvent>)
                    .after(people_enter_system::<GhostOnly, PlayerNewPositionEvent>),
                update_is_unusable_system.after(update_is_usable_system),
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
                    toggle_on_interact_system::<PlayerOnly, Ghost>,
                    toggle_on_interact_system::<GhostOnly, Player>,
                )
                    .after(update_is_unusable_system),
            )
                .run_if(in_state(GameState::InLevel)),
        );
    }
}
