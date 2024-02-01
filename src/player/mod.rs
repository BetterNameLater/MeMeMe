pub mod actions;
pub mod events {
    pub mod interact_event;
    pub mod new_position_event;
    pub mod rewind_event;
}
pub mod components {
    pub mod ghost;
    pub mod player;
}
pub mod systems {
    pub mod ghost_actions_system;
    pub mod player_input_system;
    pub mod rewind_system;
}
pub mod move_direction;
pub mod plugin;
pub use components::ghost::Ghost;
pub use systems::ghost_actions_system::{ghost_actions_system, GhostActions};
