use crate::items::primitive::player_only::PersonOnly;
use bevy::prelude::Component;

/// Items that can only interact with ghosts
#[derive(Component, Default)]
pub struct GhostOnly;

impl PersonOnly for GhostOnly {}
