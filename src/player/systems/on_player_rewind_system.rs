use crate::constantes::PLAYER_START_TRANSFORM;
use crate::level::components::level_tag::LevelTag;
use crate::player::components::player::Player;
use crate::player::events::rewind_event::RewindEvent;
use crate::player::{Ghost, GhostActions};
use crate::time::{ElapsedTimeFromStartRewind, StartTime};
use bevy::prelude::{Commands, Entity, EventReader, Query, ResMut, Transform, With, Without};

#[allow(clippy::too_many_arguments)]
pub fn on_player_rewind_system(
    mut commands: Commands,
    mut player_query: Query<&mut Player>,
    player_entity_query: Query<Entity, With<Player>>,
    mut start_time: ResMut<StartTime>,
    mut ghost_actions: ResMut<GhostActions>,
    mut elapsed_time_from_start_rewind: ResMut<ElapsedTimeFromStartRewind>,
    mut rewind_event: EventReader<RewindEvent>,
    mut player_transform_query: Query<&mut Transform, With<Player>>,
    mut ghost_transform_query: Query<&mut Transform, (With<Ghost>, Without<Player>)>,
    level_query: Query<Entity, With<LevelTag>>,
) {
    for _ in rewind_event.read() {
        let mut player = player_query.single_mut();
        ghost_actions.actions.append(&mut player.actions);
        ghost_actions.actions.sort_by(|a, b| {
            a.timestamp_seconds
                .partial_cmp(&b.timestamp_seconds)
                .unwrap()
        });
        ghost_actions.index = 0;
        player.actions.clear();

        start_time.0 = None;
        elapsed_time_from_start_rewind.0 = None;

        // reset the position of the current player, before turning im to a ghost
        *player_transform_query.single_mut() = PLAYER_START_TRANSFORM;
        commands
            .entity(player_entity_query.single())
            .remove::<Player>()
            .insert(Ghost);

        // insert a new player to replace
        Player::create_player(&mut commands, level_query.single());

        // reset ghost position
        ghost_transform_query.for_each_mut(|mut ghost_transform| {
            *ghost_transform = PLAYER_START_TRANSFORM;
        });
    }
}
