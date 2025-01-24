use bevy::prelude::*;

/// Cooldown (for button)
#[derive(Component, Default)]
pub struct Cooldown(pub Timer);

impl Cooldown {
    pub fn new() -> Self {
        Cooldown(Timer::from_seconds(1.5, TimerMode::Once))
    }
}
