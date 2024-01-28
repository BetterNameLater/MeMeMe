use crate::level::ressources::level_informations::LevelInformations;
use bevy::prelude::{Res, ResMut, Time};

pub fn elapsed_time_from_start_rewind_system(
    mut level_informations: ResMut<LevelInformations>,
    time: Res<Time>,
) {
    if level_informations.start_time.is_none() {
        return;
    }
    level_informations.elapsed_time_from_start_rewind =
        Some(time.elapsed_seconds() - level_informations.start_time.unwrap());
}
