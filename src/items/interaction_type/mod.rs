pub mod ghost_only;
pub mod player_only;

use bevy::prelude::Component;

/// Marker [`bevy::prelude::Component`] giving an indication on how we interact with it
pub trait InteractionType: Component {}
