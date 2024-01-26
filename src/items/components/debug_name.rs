use bevy::prelude::Component;

/// Component with the debug name of an item.
/// Given from the json key of the item, in the level declaration.
///
/// Example :
/// ```json
/// {
///   ...
///   "objects": {
///     "door_name": {
///         ...
///     }
///   }
/// }
/// ```
/// Here the DebugName will be "door_name"
#[derive(Component, Debug)]
pub struct DebugName(pub String);
