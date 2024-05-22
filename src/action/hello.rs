use crate::{action::UpdateAction, state::StateChange};

use super::{Action, ActionChange, BodyTick, NextTick};

const TICK_FREQUENCY: u64 = 1;

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
        state: &crate::state::State,
    ) -> (NextTick, Vec<StateChange>) {
        let mut changes = vec![];

        if self.counter == 5 {
            changes.push(StateChange::Action(id, ActionChange::Remove))
        } else {
            println!("Hello");
            changes.push(StateChange::Action(
                id,
                ActionChange::Update(UpdateAction::SayHelloFiveTimes(
                    SayHelloFiveTimesChange::IncrementCounter,
                )),
            ))
        };

        (NextTick(*state.frame_i() + TICK_FREQUENCY), changes)
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
