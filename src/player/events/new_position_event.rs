use bevy::prelude::*;

pub struct NewPositionEventData {
    pub before: IVec2,
    pub now: IVec2,
    pub entity: Entity,
}
