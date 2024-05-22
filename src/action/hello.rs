use crate::{action::UpdateAction, state::StateChange};

use super::{Action, ActionChange, BodyTick};

#[derive(Debug, PartialEq)]
pub struct SayHelloFiveTimes {
    counter: usize,
}
impl SayHelloFiveTimes {
    fn new() -> Self {
        Self { counter: 0 }
    }
}

impl BodyTick<SayHelloFiveTimesChange> for SayHelloFiveTimes {
    fn tick(
        &self,
        id: super::ActionId,
        _state: &crate::state::State,
    ) -> Vec<crate::state::StateChange> {
        let mut changes = vec![];

        if self.counter == 5 {
            changes.push(StateChange::Action(ActionChange::Remove(id)))
        } else {
            println!("Hello");
            changes.push(StateChange::Action(ActionChange::Update(
                id,
                UpdateAction::SayHelloFiveTimes(SayHelloFiveTimesChange::IncrementCounter),
            )))
        };

        changes
    }

    fn apply(&mut self, change: SayHelloFiveTimesChange) {
        match change {
            SayHelloFiveTimesChange::IncrementCounter => self.counter += 1,
        }
    }
}

pub enum SayHelloFiveTimesChange {
    IncrementCounter,
}

pub struct SayHelloFiveTimesActionBuilder;

impl SayHelloFiveTimesActionBuilder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn build(&self) -> Action {
        Action::SayHelloFiveTimes(SayHelloFiveTimes::new())
    }
}
