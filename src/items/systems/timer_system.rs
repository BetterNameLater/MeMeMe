use crate::items::components::cooldown::Cooldown;
use crate::items::components::is_activated::IsActivated;
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
