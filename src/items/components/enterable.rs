use bevy::prelude::Component;

/// Component a player or a ghost can "enter" on.
/// Will trigger [`crate::items::events::OnEnterEvent`].
#[derive(Component)]
pub struct EnterAble;
