use bevy::prelude::Component;

/// Item that can be used only one time
#[derive(Component)]
pub struct SingleUse;

// TODO
//  implement a system that checks if the item has been used ?
//  if so, add a `Used` component to flag it as not usable any more
//  to be reset at start !
