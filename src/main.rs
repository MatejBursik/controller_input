mod controller;
mod interpreter;
mod output;

use std::time::{Duration, Instant};

use controller::Controller;
use output::Output;

fn main() {
    let mut controller = Controller::new().expect("Controller not loaded");
    let mut output = Output::new().expect("Output not loaded");

    let mut previous = Instant::now();

    loop {
        let now = Instant::now();
        let dt = now - previous;
        previous = now;

        controller.update();

        let actions = interpreter::update(&controller.state, dt);

        for action in actions {
            output.execute(action).expect("Cannot execute the action");
        }

        std::thread::sleep(Duration::from_millis(5));
    }
}
