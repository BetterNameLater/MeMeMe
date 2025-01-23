use bevy::{prelude::*, time::Stopwatch};

/// Immutable level informations
#[derive(Reflect, Resource, Debug, PartialEq)]
pub struct StartPosition(IVec2);

impl StartPosition {
    pub fn new(player_start_position: IVec2) -> Self {
        Self(player_start_position)
    }

    pub fn get(&self) -> &IVec2 {
        &self.0
    }
}

/// The quantity of ghost spawned. It also reflect the number of time the rewind action was done in a row
#[derive(Reflect, Resource, Debug, PartialEq, Default)]
pub struct GhostCount(pub usize);

/// Time information for the current run. Only available in LevelState::Playing
///
/// Tuple (start_time, elapsed_time_from_start_rewind)
#[derive(Reflect, Resource, Debug, Default, PartialEq)]
pub struct PlayingTime(pub Stopwatch);
