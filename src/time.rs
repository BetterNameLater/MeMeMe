use bevy::prelude::Resource;

#[derive(Resource)]
pub struct StartTime(pub Option<f32>);

#[derive(Resource)]
pub struct ElapsedTimeFromStartRewind(pub Option<f32>);
