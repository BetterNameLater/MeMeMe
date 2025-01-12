use super::InteractionType;
use bevy::prelude::Component;

/// Mark a [`bevy::prelude::Entity`] to inform it can only interact with player
#[derive(Component)]
pub struct PlayerOnly;

impl InteractionType for PlayerOnly {}
