use bevy::prelude::Component;

/// Il t'amène dans un autre niveau, via le nom.
#[derive(Component)]
pub struct LevelTeleporter(pub String);
