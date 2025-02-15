use super::components::person::Person;
use crate::player::move_direction::MoveDirection;
use bevy::prelude::{Entity, Reflect, Resource};
use std::collections::HashMap;
use std::marker::PhantomData;
use std::time::Duration;

#[derive(Debug, Reflect, PartialEq, Clone)]
pub struct Action {
    pub ghost_entity: Entity,
    pub action_type: ActionType,
}

#[derive(Debug, Reflect, PartialEq, Clone)]
pub enum ActionType {
    Move(MoveDirection),
    Interact,
}

pub type Actions = HashMap<Duration, Vec<Action>>;

#[derive(Debug, Resource, Clone, Reflect, Default, PartialEq)]
pub struct ActionStack<T: Person> {
    new: Actions,
    played: Actions,
    #[reflect(ignore)]
    _person_marker: PhantomData<T>,
}

impl<T: Person> ActionStack<T> {
    pub fn rewind<B: Person>(&mut self, other: &mut ActionStack<B>) {
        self.played_in_new();
        other.played_in_new();
        assert_eq!(other.played.len(), 0);
        other.new.drain().for_each(|(k, v)| {
            self.insert_news(v, k);
        });
    }

    fn played_in_new(&mut self) {
        let mut played: Actions = self.played.drain().collect();
        played.drain().for_each(|(elapsed, actions)| {
            Self::insert_many(&mut self.new, elapsed, actions);
        });
    }

    fn insert(current: &mut Actions, elapsed: Duration, action: Action) {
        if let Some(actions) = current.get_mut(&elapsed) {
            actions.push(action);
        } else {
            current.insert(elapsed, vec![action]);
        }
    }

    fn insert_many(current: &mut Actions, elapsed: Duration, mut actions: Vec<Action>) {
        if let Some(existing_actions) = current.get_mut(&elapsed) {
            existing_actions.append(&mut actions);
        } else {
            current.insert(elapsed, actions);
        }
    }

    pub fn insert_new(&mut self, action: Action, elapsed: Duration) {
        Self::insert(&mut self.new, elapsed, action);
    }

    fn insert_news(&mut self, actions: Vec<Action>, elapsed: Duration) {
        Self::insert_many(&mut self.new, elapsed, actions);
    }

    fn insert_played(&mut self, actions: Vec<Action>, elapsed: Duration) {
        Self::insert_many(&mut self.played, elapsed, actions);
    }

    /// trouve la premiere liste d'action qui est dans le passé, par rapport au elapsed donné
    /// la met dans `played` et la renvoie en ref
    pub fn exec(&mut self, elapsed: Duration) -> Option<&Vec<Action>> {
        if let Some(key) = self.new.keys().find(|&&k| elapsed >= k) {
            let actions = self.new.remove(&key.clone()).unwrap();
            self.insert_played(actions, elapsed);
            self.played.get(&elapsed)
        } else {
            None
        }
    }
}
