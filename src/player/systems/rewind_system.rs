use crate::constantes::PLAYER_Z;
use crate::level::components::level_tag::LevelTag;
use crate::level::ressources::level_informations::{LevelInformations, StartPosition};
use crate::player::components::player::Player;
use crate::player::events::rewind_event::RewindEvent;
use crate::player::{Ghost, GhostActions};
use bevy::prelude::*;

#[allow(clippy::too_many_arguments)]
pub fn rewind_system(
    mut commands: Commands,
    mut player_query: Query<(Entity, &mut Player, &mut Transform, &mut Name)>,
    mut ghost_transform_query: Query<&mut Transform, (Without<Player>, With<Ghost>)>,
    level_query: Query<Entity, With<LevelTag>>,
    mut level_infos: ResMut<LevelInformations>,
    start_position: ResMut<StartPosition>,
    mut ghost_actions: ResMut<GhostActions>,
    mut rewind_event: EventReader<RewindEvent>,
) {
    if rewind_event.is_empty() {
        return;
    }
    rewind_event.clear();
    assert!(
        level_infos.elapsed_time_from_start_rewind.is_some(),
        "Rewind without actual start"
    );
    debug!("Rewind");
    let (player_entity, mut player, mut player_transform, mut player_name) =
        player_query.single_mut();
    ghost_actions.actions.append(&mut player.actions);
    ghost_actions.actions.sort_by(|a, b| {
        a.timestamp_seconds
            .partial_cmp(&b.timestamp_seconds)
            .unwrap()
    });
    ghost_actions.index = 0;

    let start_transform = start_position.get().to_transform(PLAYER_Z);

    // reset the position of the current player, before turning im to a ghost
    *player_transform = start_transform;
    commands
        .entity(player_entity)
        .remove::<Player>()
        .insert(Ghost);

    // change name
    player_name.set(format!("{}Ghost", level_infos.ghost_count));

    // insert a new player to replace
    let new_player = Player::spawn_player(&mut commands, *start_position.get());
    commands.entity(level_query.single()).add_child(new_player);

    // reset ghost position
    ghost_transform_query
        .iter_mut()
        .for_each(|mut ghost_transform| {
            *ghost_transform = start_transform;
        });

    level_infos.rewind();
}
