use std::{thread, time::Duration};

use action::{hello::SayHelloFiveTimesActionBuilder, ActionChange, ActionId};
use state::{State, StateChange};

mod action;
mod state;

fn main() {
    let mut state = State::new();
    state.apply(vec![StateChange::Action(ActionChange::New(
        ActionId::new(),
        SayHelloFiveTimesActionBuilder::new().build(),
    ))]);

    loop {
        // let (update_a, update_b) = thread::scope(|scope| {
        //     let update_a = scope.spawn(|| job_a(&state));
        //     let update_b = scope.spawn(|| job_b(&state));
        //     (update_a.join(), update_b.join())
        // });

        let mut changes = vec![];
        for (action_id, action) in state.actions() {
            changes.extend(action.tick(*action_id, &state))
        }
        state.apply(changes);

        // dbg!(&state);
        thread::sleep(Duration::from_millis(500));
    }
}
