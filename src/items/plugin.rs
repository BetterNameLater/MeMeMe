use crate::game_state::GameState;
use crate::items::interaction_type::ghost_only::GhostOnly;
use crate::items::interaction_type::player_only::PlayerOnly;
use crate::items::systems::button_system::button_pressed_system;
use crate::items::systems::count_people_on_system::count_people_on_system;
use crate::items::systems::level_teleporter_system::level_teleporter_system;
use crate::items::systems::teleporter_system::teleporter_system;
use crate::items::systems::timer_system::cooldown_system;
use crate::items::systems::timer_system::start_timer_system;
use crate::items::systems::toggle_on_system::{toggle_on_enter_system, toggle_on_interact_system};
use crate::items::systems::transitions::enter_rewind;
use crate::items::systems::update_is_usable_system::{
    update_is_unusable_system, update_is_usable_system,
};
use crate::items::systems::visual_system::visual_system;
use crate::level::level_state::LevelState;
use crate::player::actions_system;
use crate::player::components::player::Player;
use crate::player::Ghost;
use bevy::prelude::*;

pub struct ItemsPlugin;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                start_timer_system, // TODO order
                visual_system,
                cooldown_system
                    .after(actions_system::<Player, GhostOnly>)
                    .after(actions_system::<Ghost, PlayerOnly>),
                update_is_usable_system.after(cooldown_system),
                update_is_unusable_system.after(update_is_usable_system),
            )
                .run_if(in_state(GameState::InLevel)),
        )
        .add_systems(
            Update,
            (
                level_teleporter_system,
                button_pressed_system::<GhostOnly, Player>,
                button_pressed_system::<PlayerOnly, Ghost>,
                count_people_on_system::<GhostOnly, Player>,
                count_people_on_system::<PlayerOnly, Ghost>,
                teleporter_system::<PlayerOnly, Ghost>,
                teleporter_system::<GhostOnly, Player>,
                toggle_on_enter_system::<PlayerOnly, Ghost>,
                toggle_on_enter_system::<GhostOnly, Player>,
                toggle_on_interact_system::<PlayerOnly, Ghost>,
                toggle_on_interact_system::<GhostOnly, Player>,
            )
                .run_if(in_state(LevelState::Playing))
                .after(update_is_unusable_system),
        );
        self.register_transitions(app);
    }
}

impl ItemsPlugin {
    fn register_transitions(&self, app: &mut App) {
        app.add_systems(OnEnter(LevelState::Rewind), enter_rewind);
    }
}
