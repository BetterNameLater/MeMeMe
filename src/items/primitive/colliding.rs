use bevy::prelude::Component;

/// Component a person can collide with.
/// Used as wall or with [`crate::items::door::Door`].
///
/// We can consider than if [`crate::items::primitive::is_usable::IsUsable`] is in the Entity, it doesn't collide
#[derive(Component, Default)]
pub struct Colliding;
