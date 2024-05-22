pub mod hello;
use uuid::Uuid;

use crate::state::{State, StateChange};

use self::hello::{SayHelloFiveTimes, SayHelloFiveTimesChange};

#[derive(Debug, PartialEq)]
pub enum Action {
    SayHelloFiveTimes(SayHelloFiveTimes),
}

impl Action {
    pub fn tick(&self, id: ActionId, state: &State) -> Vec<StateChange> {
        match self {
            Self::SayHelloFiveTimes(body) => body.tick(id, state),
        }
    }

    pub fn apply(&mut self, change: UpdateAction) {
        match (self, change) {
            (Self::SayHelloFiveTimes(body), UpdateAction::SayHelloFiveTimes(action_change)) => {
                body.apply(action_change)
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ActionId(Uuid);
impl ActionId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

pub enum ActionChange {
    New(ActionId, Action),
    Update(ActionId, UpdateAction),
    Remove(ActionId),
}

pub enum UpdateAction {
    SayHelloFiveTimes(SayHelloFiveTimesChange),
}

pub trait BodyTick<T> {
    // fn from()
    fn tick(&self, id: ActionId, _state: &State) -> Vec<StateChange>;
    fn apply(&mut self, change: T);
}
