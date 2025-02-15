use bevy::prelude::*;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameState {
    #[default]
    BootingGame,
    Menu,
    LoadingLevel,
    InLevel,
    ErrorInitialLoad,
}
