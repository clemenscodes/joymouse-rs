use crate::{Axis, JoyStick, Polarity, State};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct JoyStickEvent {
  joystick: JoyStick,
  axis: Axis,
  polarity: Polarity,
  state: State,
}

impl JoyStickEvent {
  pub fn new(joystick: JoyStick, axis: Axis, polarity: Polarity, state: State) -> Self {
    Self {
      joystick,
      axis,
      polarity,
      state,
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

  pub fn state(&self) -> State {
    self.state
  }
}
