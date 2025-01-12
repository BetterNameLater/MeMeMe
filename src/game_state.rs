use bevy::prelude::*;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameState {
    #[default]
    BootingGame,
    LoadingLevel,
    InLevel,
    ErrorInitialLoad,
}
