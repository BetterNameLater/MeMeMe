use bevy::prelude::{Component, Entity};

/// List all the dependencies of an item
/// Permets de définir de quelles entités un item dépend pour être utilisable.
#[derive(Component)]
pub struct Dependencies(pub Vec<Entity>);

// use std::marker::PhantomData;
// pub struct On;
// pub struct Off;
// TODO: on se rend compte tout de suite que ça ne peut pas check que l'item ne soit pas "IsOn".
// (donc est-ce qu'on veut un système de dépendance négative)
