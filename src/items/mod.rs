mod pressure_plate;
mod event;



use bevy::ecs::component::Component;
pub use event::on_enter_system;

#[derive(Component)]
pub struct Population(pub u32);

#[derive(Component)]
pub struct Enterable;