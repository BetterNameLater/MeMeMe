use bevy::prelude::Component;

/// Component a player or a ghost can "interact" with.
/// Will trigger [`crate::items::events::OnEnterEvent`].
///
/// This is similare to [`crate::items::primitive::Enterable`].
#[derive(Component, Default)]
pub struct Interactible;
