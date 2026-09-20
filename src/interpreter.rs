use std::time::Duration;

use crate::controller::ControllerState;
use crate::output::Action;

pub fn update(state: &ControllerState, _dt: Duration) -> Vec<Action> {
    let mut actions = Vec::new();

    // Speed
    let speed = if state.left_bumper || state.right_bumper {
        2.0
    } else {
        5.0
    };

    // Movement
    let x = deadzone(state.left_x, 0.15);
    let y = deadzone(state.left_y, 0.15);

    actions.push(
        Action::MouseMove {
            dx: (x * speed) as i32,
            dy: (-y * speed) as i32 // negative so it is not inverted
        }
    );

    // Left click
    if state.south {
        actions.push(
            Action::LeftClick
        );
    }

    // Left click
    if state.east {
        actions.push(
            Action::RightClick
        );
    }

    actions
}

fn deadzone(value: f32, threshold: f32) -> f32 {
    if value.abs() < threshold {
        0.0
    } else {
        value
    }
}