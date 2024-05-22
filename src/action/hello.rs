use crate::{action::UpdateAction, state::StateChange};

use super::{Action, ActionChange, BodyTick, NextTick};

const TICK_FREQUENCY: u64 = 1;

#[derive(Debug, PartialEq)]
pub struct SayHello {
    counter: usize,
}

impl SayHello {
    fn new() -> Self {
        Self { counter: 0 }
    }
}

impl BodyTick<SayHelloChange> for SayHello {
    fn tick(
        &self,
        id: super::ActionId,
        state: &crate::state::State,
    ) -> (NextTick, Vec<StateChange>) {
        let mut changes = vec![];

        if self.counter == 5 {
            changes.push(StateChange::Action(id, ActionChange::Remove))
        } else {
            // println!("Hello");
            changes.push(StateChange::Action(
                id,
                ActionChange::Update(UpdateAction::SayHello(SayHelloChange::IncrementCounter)),
            ))
        };

        (NextTick(*state.frame_i() + TICK_FREQUENCY), changes)
    }

    fn apply(&mut self, change: SayHelloChange) {
        match change {
            SayHelloChange::IncrementCounter => self.counter += 1,
        }
    }
}

pub enum SayHelloChange {
    IncrementCounter,
}

pub struct SayHelloActionBuilder;

impl SayHelloActionBuilder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn build(&self) -> Action {
        Action::SayHello(SayHello::new())
    }
}
