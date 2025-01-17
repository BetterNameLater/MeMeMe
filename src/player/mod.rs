pub mod actions;
pub mod events {
    pub mod interact_event;
    pub mod new_position_event;
}
pub mod components {
    pub mod ghost;
    pub mod person;
    pub mod player;
}
pub mod systems {
    pub mod actions_system;
    pub mod input_system;
    pub mod transitions;
}
pub mod move_direction;
pub mod plugin;
pub use components::ghost::Ghost;
pub use systems::actions_system::actions_system;
