use bevy::prelude::Component;

/// Certains items ne peuvent interagir qu'avec le joueur.
#[derive(Component, Default)]
pub struct PlayerOnly;

pub trait PersonOnly: Component + Default {}

impl PersonOnly for PlayerOnly {}
