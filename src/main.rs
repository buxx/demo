use action::{hello::SayHelloFiveTimesActionBuilder, ActionId};
use run::RunnerBuilder;

mod action;
mod run;
mod state;

// TODO: choice + parralel
fn main() {
    RunnerBuilder::new()
        .actions(vec![(
            ActionId::new(),
            SayHelloFiveTimesActionBuilder::new().build(),
        )])
        .build()
        .run();
}
