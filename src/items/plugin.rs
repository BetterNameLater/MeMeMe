use crate::game_state::GameState;
use crate::items::interaction_type::ghost_only::GhostOnly;
use crate::items::interaction_type::player_only::PlayerOnly;
use crate::items::reset_level_items::reset_level_items;
use crate::items::systems::button_system::button_pressed_system;
use crate::items::systems::count_people_on_system::count_people_on_system;
use crate::items::systems::level_teleporter_system::level_teleporter_system;
use crate::items::systems::teleporter_system::{teleporter_activate_system, teleporter_system};
use crate::items::systems::timer_system::cooldown_system;
use crate::items::systems::timer_system::start_timer_system;
use crate::items::systems::toggle_on_system::{toggle_on_enter_system, toggle_on_interact_system};
use crate::items::systems::update_is_usable_system::{
    update_is_unusable_system, update_is_usable_system,
};
use crate::items::systems::visual_system::visual_system;
use crate::player::components::player::Player;
use crate::player::systems::player_input_system::player_action_input_system;
use crate::player::systems::player_input_system::player_move_input_system;
use crate::player::Ghost;
use bevy::prelude::*;

pub struct ItemsPlugin;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                reset_level_items,
                start_timer_system, // TODO order
                cooldown_system
                    .after(player_move_input_system)
                    .after(player_action_input_system),
                update_is_usable_system.after(cooldown_system),
                update_is_unusable_system.after(update_is_usable_system),
                (
                    level_teleporter_system,
                    visual_system,
                    button_pressed_system::<GhostOnly, Player>,
                    button_pressed_system::<PlayerOnly, Ghost>,
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
