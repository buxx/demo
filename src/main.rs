use action::{hello::SayHelloActionBuilder, ActionId};
use run::RunnerBuilder;

mod action;
mod run;
mod state;

// TODO: choice + parralel
fn main() {
    let mut actions = vec![];
    for _ in 0..10_000 {
        actions.push((ActionId::new(), SayHelloActionBuilder::new().build()));
    }

    RunnerBuilder::new().actions(actions).build().run();
}
