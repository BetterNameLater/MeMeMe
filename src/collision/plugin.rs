use crate::collision::ressources::collision_map::CollisionMap;
use bevy::prelude::*;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CollisionMap::default());
    }
}
