use crate::constantes::PLAYER_Z;
use crate::items::events::OnEnterEvent;
use crate::items::interaction_type::InteractionType;
use crate::items::primitive::enterable::EnterAble;
use crate::items::primitive::is_usable::IsUsable;
use crate::items::teleporter::Teleporter;
use crate::player::components::person::Person;
use bevy::prelude::*;

pub fn teleporter_system<W: InteractionType, T: Person>(
    teleporter_query: Query<&Teleporter, (With<IsUsable>, Without<W>, With<EnterAble>)>,
    mut person_query: Query<&mut Transform, With<T>>,
    mut on_enter_event_reader: EventReader<OnEnterEvent>,
) {
    for on_enter_event in on_enter_event_reader.read() {
        if let Ok(mut person) = person_query.get_mut(on_enter_event.person) {
            if let Ok(teleporter) = teleporter_query.get(on_enter_event.item) {
                person.translation =
                    Vec3::new(teleporter.0.x as f32, teleporter.0.y as f32, PLAYER_Z);
            }
        }
    }
}
