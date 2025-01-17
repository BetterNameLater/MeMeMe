use super::person::Person;
use bevy::{prelude::Component, reflect::Reflect};

#[derive(Component, Debug, Default, Clone, Reflect)]
pub struct Ghost;

impl Person for Ghost {}
