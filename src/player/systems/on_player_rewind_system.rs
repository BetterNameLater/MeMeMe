use crate::constantes::PLAYER_Z;
use crate::level::components::level_tag::LevelTag;
use crate::level::ressources::level_informations::LevelInformations;

use crate::player::components::player::Player;
use crate::player::events::rewind_event::RewindEvent;
use crate::player::{Ghost, GhostActions};
use crate::time::{ElapsedTimeFromStartRewind, StartTime};
use bevy::hierarchy::BuildChildren;
use bevy::log::debug;
use bevy::prelude::{Commands, Entity, EventReader, Query, Res, ResMut, Transform, With, Without};

#[allow(clippy::too_many_arguments)]
pub fn on_player_rewind_system(
    mut commands: Commands,
    mut player_query: Query<&mut Player>,
    player_entity_query: Query<Entity, With<Player>>,
    mut start_time: ResMut<StartTime>,
    mut elapsed_time_from_start_rewind: ResMut<ElapsedTimeFromStartRewind>,
    mut ghost_actions: ResMut<GhostActions>,
    mut rewind_event: EventReader<RewindEvent>,
    mut player_transform_query: Query<&mut Transform, With<Player>>,
    mut ghost_transform_query: Query<&mut Transform, (With<Ghost>, Without<Player>)>,
    level_query: Query<Entity, With<LevelTag>>,
    level_infos: Res<LevelInformations>,
) {
    for _ in rewind_event.read() {
        debug!("Rewind");
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

        let start_transform = level_infos
            .player_start_position
            .to_transform(PLAYER_Z as i32);

        // reset the position of the current player, before turning im to a ghost
        *player_transform_query.single_mut() = start_transform;
        commands
            .entity(player_entity_query.single())
            .remove::<Player>()
            .insert(Ghost);

        // insert a new player to replace
        let new_player = Player::spawn_player(&mut commands, level_infos.player_start_position);
        commands.entity(level_query.single()).add_child(new_player);

        // reset ghost position
        ghost_transform_query.for_each_mut(|mut ghost_transform| {
            *ghost_transform = start_transform;
        });
    }
}
