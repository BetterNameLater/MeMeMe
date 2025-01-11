use super::InteractionType;
use bevy::prelude::Component;

/// Items that can only interact with ghosts
#[derive(Component, Default)]
pub struct GhostOnly;

impl InteractionType for GhostOnly {}
