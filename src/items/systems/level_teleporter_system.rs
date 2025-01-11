use crate::items::events::OnEnterEvent;
use crate::items::level_teleporter::LevelTeleporter;
use crate::items::primitive::enterable::EnterAble;
use crate::items::primitive::is_usable::IsUsable;
use crate::level::components::level_to_go::LevelToGo;
use crate::state::GameState;
use bevy::prelude::*;

pub fn level_teleporter_system(
    mut on_enter_event_reader: EventReader<OnEnterEvent>,
    mut commands: Commands,
    level_teleporter_query: Query<&LevelTeleporter, (With<EnterAble>, With<IsUsable>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for on_enter_event in on_enter_event_reader.read() {
        if let Ok(level_name) = level_teleporter_query.get(on_enter_event.item) {
            commands.spawn(LevelToGo(level_name.0.to_string()));
            next_state.set(GameState::LoadingLevel);
        }
    }
}
