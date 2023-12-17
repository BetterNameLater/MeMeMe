use bevy::prelude::{Res, ResMut, Resource, Time};

#[derive(Resource)]
pub struct StartTime(pub Option<f32>);

#[derive(Resource)]
pub struct ElapsedTimeFromStartRewind(pub Option<f32>);

pub fn elapsed_time_from_start_rewind_system(
    mut elapsed_time_from_start_rewind: ResMut<ElapsedTimeFromStartRewind>,
    start_time: Res<StartTime>,
    time: Res<Time>,
) {
    if start_time.0.is_none() {
        return;
    }
    elapsed_time_from_start_rewind.0 = Some(time.elapsed_seconds() - start_time.0.unwrap());
}
