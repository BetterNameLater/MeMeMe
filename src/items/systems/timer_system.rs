use crate::items::primitive::cooldown::Cooldown;
use crate::items::primitive::is_activated::IsActivated;
use crate::items::primitive::is_usable::IsUsable;
use crate::items::primitive::start_timer::StartTimer;
use bevy::prelude::*;

pub fn cooldown_system(
    mut commands: Commands,
    time: Res<Time>,
    mut cooldown_query: Query<(Entity, &mut IsActivated, &mut Cooldown<IsActivated>)>,
) {
    for (entity, mut is_activated, mut timer) in cooldown_query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            is_activated.0 = false;
            commands.entity(entity).remove::<Cooldown<IsActivated>>();
            debug!("Cooldown ended, removing it.");
        }
    }
}

pub fn start_timer_system(
    mut commands: Commands,
    time: Res<Time>,
    mut start_time_query: Query<(Entity, &mut StartTimer), With<IsUsable>>,
) {
    for (entity, mut timer) in start_time_query.iter_mut() {
        if timer.0.finished() {
            // TODO to be run after all
            commands.entity(entity).remove::<IsUsable>();
            debug!("StartTimer ended, blocking it.");
            continue;
        }
        timer.0.tick(time.delta());
    }
}
