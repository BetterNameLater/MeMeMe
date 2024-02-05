use bevy::prelude::*;
use std::marker::PhantomData;

/// Cooldown (for button)
#[derive(Component, Default)]
pub struct Cooldown<T>(pub Timer, PhantomData<T>);

impl<T> Cooldown<T> {
    pub fn new(duration: f32) -> Self {
        Cooldown(Timer::from_seconds(duration, TimerMode::Once), PhantomData)
    }
}
