use crate::items::components::enterable::EnterAble;
use crate::items::components::is_usable::IsUsable;
use crate::items::components::player_only::PersonOnly;
use crate::items::components::teleporter::Teleporter;
use crate::items::events::OnEnterEvent;
use crate::player::components::player::Person;
use bevy::prelude::*;
use bevy::utils::HashSet;

pub fn teleporter_system<W: PersonOnly, T: Person>(
    teleporter_query: Query<&Teleporter, (With<IsUsable>, Without<W>, With<EnterAble>)>,
    mut person_query: Query<&mut Transform, With<T>>,
    mut on_enter_event_reader: EventReader<OnEnterEvent>,
) {
    for on_enter_event in on_enter_event_reader.read() {
        if let Ok(mut person) = person_query.get_mut(on_enter_event.person) {
            if let Ok(teleporter) = teleporter_query.get(on_enter_event.item) {
                person.translation = Vec3::new(teleporter.0.x as f32, teleporter.0.y as f32, 1.);
            }
        }
    }
}

// TODO Might be Refactored
// teleport all entities already on the teleporter if it's activated
pub fn teleporter_activate_system<W: PersonOnly, T: Person>(
    teleporter_query: Query<(&Teleporter, &Transform), (Changed<IsUsable>, Without<W>)>,
    mut entities_query: Query<(Entity, &mut Transform), (With<T>, Without<Teleporter>)>,
) {
    let mut moved: HashSet<Entity> = HashSet::new();
    for (dest, src) in teleporter_query.iter() {
        let mut entities: Vec<(Entity, Mut<'_, Transform>)> = entities_query
            .iter_mut()
            .filter(|(_, e)| {
                e.translation.x == src.translation.x && e.translation.y == src.translation.y
            })
            .collect();
        for (entity, transform) in entities.iter_mut() {
            if moved.contains(entity) {
                continue;
            }
            transform.translation = Vec3::new(dest.0.x as f32, dest.0.y as f32, 1.);
            moved.insert(*entity);
        }
    }
}
