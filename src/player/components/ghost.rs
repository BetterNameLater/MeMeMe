use super::person::Person;
use bevy::prelude::Component;

#[derive(Component, Debug, Default)]
pub struct Ghost;

impl Person for Ghost {}
