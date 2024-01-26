use bevy::prelude::Component;

/// Certains items ne peuvent interagir qu'avec le joueur.
#[derive(Component)]
pub struct PlayerOnly;
