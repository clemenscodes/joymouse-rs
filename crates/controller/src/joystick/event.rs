use crate::{JoyStick, JoyStickAxis, Polarity};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct JoyStickEvent {
  joystick: JoyStick,
  axis: JoyStickAxis,
  polarity: Polarity,
}

impl JoyStickEvent {
  pub fn new(joystick: JoyStick, axis: JoyStickAxis, polarity: Polarity) -> Self {
    Self {
      joystick,
      axis,
      polarity,
    }
  }

  pub fn joystick(&self) -> &JoyStick {
    &self.joystick
  }

  pub fn axis(&self) -> &JoyStickAxis {
    &self.axis
  }

  pub fn polarity(&self) -> Polarity {
    self.polarity
  }
}
