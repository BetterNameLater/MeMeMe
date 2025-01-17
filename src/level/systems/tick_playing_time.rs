use crate::level::ressources::level_informations::PlayingTime;
use bevy::prelude::{Res, ResMut, Time};

pub fn tick_playing_time(mut playing_time: ResMut<PlayingTime>, time: Res<Time>) {
    playing_time.0.tick(time.delta());
}
