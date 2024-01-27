use crate::items::components::debug_name::DebugName;
use crate::items::components::is_activated::IsActivated;
use crate::items::components::is_usable::IsUsable;
use bevy::prelude::Bundle;

/// Shared properties between all items
#[derive(Bundle)]
pub struct ItemBundle {
    pub is_activated: IsActivated,
    pub is_usable: IsUsable,
    pub debug_name: DebugName,
}
