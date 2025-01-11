use super::InteractionType;
use bevy::prelude::Component;

/// Certains items ne peuvent interagir qu'avec le joueur.
#[derive(Component, Default)]
pub struct PlayerOnly;

impl InteractionType for PlayerOnly {}
