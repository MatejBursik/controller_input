use gilrs::{ Axis, Button, Event, EventType, Gilrs };

#[derive(Default, Debug)]
pub struct ControllerState {
    pub left_x: f32,
    pub left_y: f32,
    pub right_x: f32,
    pub right_y: f32,

    pub south: bool,
    pub east: bool,
    pub west: bool,
    pub north: bool,

    pub left_bumper: bool,
    pub right_bumper: bool
}

pub struct Controller {
    gilrs: Gilrs,
    pub state: ControllerState
}

impl Controller {
    pub fn new() -> Result<Self, gilrs::Error> {
        Ok(Self {
            gilrs: Gilrs::new()?,
            state: ControllerState::default()
        })
    }

    pub fn update(&mut self) {
        while let Some(Event { event, .. }) = self.gilrs.next_event() {
            match event {
                EventType::AxisChanged(axis, value, _) => {
                    match axis {
                        Axis::LeftStickX => self.state.left_x = value,
                        Axis::LeftStickY => self.state.left_y = value,
                        Axis::RightStickX => self.state.right_x = value,
                        Axis::RightStickY => self.state.right_y = value,
                        _ => {}
                    }
                }

                EventType::ButtonPressed(button, _) => {
                    self.set_button(button, true);
                }

                EventType::ButtonReleased(button, _) => {
                    self.set_button(button, false);
                }

                _ => {}
            }
        }
    }

    fn set_button(&mut self, button: Button, pressed: bool) {
        match button {
            Button::South => self.state.south = pressed,
            Button::East => self.state.east = pressed,
            Button::West => self.state.west = pressed,
            Button::North => self.state.north = pressed,

            Button::LeftTrigger => self.state.left_bumper = pressed,
            Button::RightTrigger => self.state.right_bumper = pressed,

            _ => {}
        }
    }
}
