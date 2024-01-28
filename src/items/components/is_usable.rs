use bevy::prelude::Component;

/// En fonction de certaines conditions, un item peut être activé, donc utilisable.
/// Il ne prend pas de boolean. Le fait d'avoir ce tag en soi rend l'item utilisable. Cela permet de ne query les items utilisables.
#[derive(Component)]
pub struct IsUsable;
