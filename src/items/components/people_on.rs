use bevy::prelude::Component;

/// The number of [`Ghost`] and [`Player`] on this position
/// Used for the pressure plate item
#[derive(Component, Default)]
pub struct PeopleOn(pub usize);
