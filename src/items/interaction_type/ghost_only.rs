use super::InteractionType;
use bevy::prelude::Component;

/// Mark a [`bevy::prelude::Entity`] to inform it can only interact with ghosts
#[derive(Component)]
pub struct GhostOnly;

impl InteractionType for GhostOnly {}
