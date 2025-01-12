use crate::level::ressources::level_informations::PlayingTime;
use bevy::prelude::{Res, ResMut, Time};

pub fn elapsed_time_from_start_rewind_system(
    mut playing_time: ResMut<PlayingTime>,
    time: Res<Time>,
) {
    playing_time.1 = time.elapsed_secs() - playing_time.0;
}
