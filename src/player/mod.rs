pub mod actions;
pub mod events {
    pub mod ghost_new_position_event;
    pub mod new_position_event;
    pub mod player_new_position_event;
    pub mod rewind_event;
}
pub mod ghost;
pub mod ghost_actions;
pub mod interact;
pub mod move_direction;
pub mod player;
// exports

pub use events::ghost_new_position_event::GhostNewPositionEvent;
pub use events::player_new_position_event::PlayerNewPositionEvent;
pub use ghost::Ghost;
pub use ghost_actions::{ghost_actions_system, GhostActions};
