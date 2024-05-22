use std::{thread, time::Duration};

use crate::{
    action::{Action, ActionChange, ActionId},
    state::{State, StateChange},
};

pub struct Runner {
    state: State,
}

impl Runner {
    pub fn new(state: State) -> Self {
        Runner { state }
    }

    pub fn run(mut self) {
        loop {
            // let (update_a, update_b) = thread::scope(|scope| {
            //     let update_a = scope.spawn(|| job_a(&state));
            //     let update_b = scope.spawn(|| job_b(&state));
            //     (update_a.join(), update_b.join())
            // });

            let mut changes = vec![];
            for (action_id, action) in self.state.actions() {
                changes.extend(action.tick(*action_id, &self.state))
            }
            self.state.apply(changes);

            // dbg!(&state);
            thread::sleep(Duration::from_millis(500));
        }
    }
}

pub struct RunnerBuilder {
    actions: Vec<(ActionId, Action)>,
}

impl RunnerBuilder {
    pub fn new() -> Self {
        Self { actions: vec![] }
    }

    pub fn actions(mut self, value: Vec<(ActionId, Action)>) -> Self {
        self.actions = value;
        self
    }

    pub fn build(self) -> Runner {
        let mut state = State::new();

        for (action_id, action) in self.actions {
            state.apply(vec![StateChange::Action(ActionChange::New(
                action_id, action,
            ))]);
        }

        Runner::new(state)
    }
}
