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

            let mut state_changes = vec![];
            for (action_id, action) in self.state.actions() {
                let (next, changes) = action.tick(*action_id, &self.state);
                state_changes.push(StateChange::Action(
                    *action_id,
                    ActionChange::SetNextTick(next),
                ));
                // NOTE: It is important than SetNextTick is before because changes
                // can contains action deletion
                state_changes.extend(changes);
            }

            self.state.apply(state_changes);
            self.state.increment();

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
            state.apply(vec![StateChange::Action(
                action_id,
                ActionChange::New(action),
            )]);
        }

        Runner::new(state)
    }
}
