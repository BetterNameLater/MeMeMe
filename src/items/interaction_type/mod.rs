pub mod ghost_only;
pub mod player_only;

use bevy::prelude::Component;

pub trait InteractionType: Component + Default {}
