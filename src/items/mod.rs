pub mod events;
pub mod plugin;
pub mod populate_items;
pub mod systems {
    pub mod button_system;
    pub mod cooldown_system;
    pub mod count_people_on_system;
    pub mod level_teleporter_system;
    pub mod teleporter_system;
    pub mod toggle_on_system;
    pub mod transitions;
    pub mod update_is_usable_system;
    pub mod visual_system;
}
pub mod primitive {
    pub mod colliding;
    pub mod cooldown;
    pub mod dependencies;
    pub mod enterable;
    pub mod interactible;
    pub mod is_activated;
    pub mod is_usable;
    pub mod item;
    pub mod people_on;
    pub mod pressable;
    pub mod single_use;
    pub mod toggle;
}
pub mod interaction_type;

pub mod button;
pub mod door;
pub mod level_teleporter;
pub mod lever;
pub mod pressure_plate;
pub mod teleporter;
pub mod win;
