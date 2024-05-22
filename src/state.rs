use std::collections::HashMap;

use crate::action::{Action, ActionChange, ActionId};

pub struct State {
    frame_i: FrameI,
    actions: HashMap<ActionId, Action>,
}

impl State {
    pub fn apply(&mut self, changes: Vec<StateChange>) {
        for change in changes {
            match change {
                StateChange::Action(ActionChange::New(id, action)) => {
                    self.actions.insert(id, action);
                }
                StateChange::Action(ActionChange::Update(id, change)) => {
                    self.actions.get_mut(&id).unwrap().apply(change);
                }
                StateChange::Action(ActionChange::Remove(id)) => {
                    self.actions.remove(&id);
                }
            };
        }
    }

    pub fn new() -> Self {
        Self {
            frame_i: FrameI(0),
            actions: HashMap::new(),
        }
    }

    pub fn actions(&self) -> &HashMap<ActionId, Action> {
        &self.actions
    }
}

pub enum StateChange {
    Action(ActionChange),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FrameI(u64);
