use crate::constantes::PLAYER_Z;
use crate::level::components::level_tag::LevelTag;
use crate::level::ressources::level_informations::{GhostCount, PlayingTime, StartPosition};
use crate::player::components::player::Player;
use crate::player::{Ghost, GhostActions};
use bevy::prelude::*;

#[allow(clippy::too_many_arguments)]
pub fn enter_rewind(
    mut commands: Commands,
    player_query: Query<(Entity, &mut Player, &mut Transform, &mut Name)>,
    ghost_transform_query: Query<&mut Transform, (Without<Player>, With<Ghost>)>,
    level_query: Query<Entity, With<LevelTag>>,
    ghost_count: ResMut<GhostCount>,
    start_position: ResMut<StartPosition>,
    ghost_actions: ResMut<GhostActions>,
) {
    commands.remove_resource::<PlayingTime>();

    rewind_system(
        commands,
        player_query,
        ghost_transform_query,
        level_query,
        ghost_count,
        start_position,
        ghost_actions,
    );
}

pub fn enter_playing(mut commands: Commands) {
    commands.init_resource::<PlayingTime>();
}

#[allow(clippy::too_many_arguments)]
fn rewind_system(
    mut commands: Commands,
    mut player_query: Query<(Entity, &mut Player, &mut Transform, &mut Name)>,
    mut ghost_transform_query: Query<&mut Transform, (Without<Player>, With<Ghost>)>,
    level_query: Query<Entity, With<LevelTag>>,
    mut ghost_count: ResMut<GhostCount>,
    start_position: ResMut<StartPosition>,
    mut ghost_actions: ResMut<GhostActions>,
) {
    debug!("Rewind");
    let (player_entity, mut player, mut player_transform, mut player_name) =
        player_query.single_mut();

    let GhostActions { actions } = ghost_actions.clone(); // TODO remove clooone
    *ghost_actions = GhostActions {
        actions: actions.rewind(&mut player.actions),
    };

    let start_transform = start_position.get().to_transform(PLAYER_Z);

    // reset the position of the current player, before turning im to a ghost
    *player_transform = start_transform;
    commands
        .entity(player_entity)
        .remove::<Player>()
        .insert(Ghost);

    // change name
    player_name.set(format!("{}Ghost", ghost_count.0));

    // insert a new player to replace
    let new_player = Player::spawn_player(&mut commands, *start_position.get());
    commands.entity(level_query.single()).add_child(new_player);

    // reset ghost position
    ghost_transform_query
        .iter_mut()
        .for_each(|mut ghost_transform| {
            *ghost_transform = start_transform;
        });

    ghost_count.0 += 1;
}
