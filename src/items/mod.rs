pub mod debug_name;
pub mod dependencies;
pub mod door;
pub mod enterable;
pub mod events;
pub mod ghost_only;
pub mod is_usable;
pub mod player_only;
pub mod populate_items;

pub mod systems {
    pub mod is_activated;
    pub mod people_on;
    pub mod teleport;
    pub mod toggle;
}
