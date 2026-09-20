use enigo::{ Button, Coordinate, Direction, Enigo, Mouse, Settings };

#[derive(Debug)]
pub enum Action {
    MouseMove { dx: i32, dy: i32 },
    LeftClick,
    RightClick
}

pub struct Output {
    enigo: Enigo
}

impl Output {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            enigo: Enigo::new(&Settings::default())?
        })
    }

    pub fn execute(&mut self, action: Action) -> anyhow::Result<()> {
        match action {
            Action::MouseMove { dx, dy } => {
                self.enigo.move_mouse(dx, dy, Coordinate::Rel)?;
            }

            Action::LeftClick => {
                self.enigo.button(Button::Left, Direction::Click)?;
            }

            Action::RightClick => {
                self.enigo.button(Button::Right, Direction::Click)?;
            }
        }

        Ok(())
    }
}
