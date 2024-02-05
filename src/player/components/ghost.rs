use crate::player::components::player::Person;
use bevy::prelude::*;

#[derive(Component, Debug, Default)]
pub struct Ghost;

impl Person for Ghost {}
