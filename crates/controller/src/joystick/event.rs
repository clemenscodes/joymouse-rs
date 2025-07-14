use crate::{Axis, JoyStick, Polarity};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct JoyStickEvent {
  joystick: JoyStick,
  axis: Axis,
  polarity: Polarity,
}

impl JoyStickEvent {
  pub fn new(joystick: JoyStick, axis: Axis, polarity: Polarity) -> Self {
    Self {
      joystick,
      axis,
      polarity,
    }
  }

  pub fn joystick(&self) -> &JoyStick {
    &self.joystick
  }

  pub fn axis(&self) -> &Axis {
    &self.axis
  }

  pub fn polarity(&self) -> Polarity {
    self.polarity
  }
}
