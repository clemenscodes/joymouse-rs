use evdev::{AbsoluteAxisCode, KeyEvent, RelativeAxisEvent};

use crate::controller::{
  button::ControllerButton,
  joystick::{JOYSTICK_KEYS, JoyStick, JoyStickError, axis::JoyStickAxis, polarity::Polarity},
  state::State,
};

#[derive(Debug)]
pub struct ControllerJoyStickEvent {
  joystick: JoyStick,
  axis: JoyStickAxis,
  polarity: Polarity,
  state: State,
}

impl ControllerJoyStickEvent {
  pub fn new(joystick: JoyStick, axis: JoyStickAxis, polarity: Polarity, state: State) -> Self {
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

  pub fn axis(&self) -> &JoyStickAxis {
    &self.axis
  }

  pub fn polarity(&self) -> Polarity {
    self.polarity
  }

  pub fn state(&self) -> &State {
    &self.state
  }
}

impl TryFrom<KeyEvent> for ControllerJoyStickEvent {
  type Error = JoyStickError;

  fn try_from(value: KeyEvent) -> Result<Self, Self::Error> {
    let code = value.code();
    let joystick = JoyStick::Left;
    let axis = JoyStickAxis::try_from((*JOYSTICK_KEYS, code))?;
    let button = ControllerButton::try_from(code)?;
    let state = State::try_from(value.value())?;
    let polarity = Polarity::try_from((&axis, &button, code))?;
    Ok(Self::new(joystick, axis, polarity, state))
  }
}

impl TryFrom<RelativeAxisEvent> for ControllerJoyStickEvent {
  type Error = JoyStickError;

  fn try_from(value: RelativeAxisEvent) -> Result<Self, Self::Error> {
    let (code, value) = value.destructure();
    let joystick = JoyStick::try_from(code)?;
    let axis = JoyStickAxis::try_from(code)?;
    let polarity = Polarity::try_from(value as f64)?;
    let state = State::Pressed;
    Ok(Self::new(joystick, axis, polarity, state))
  }
}

impl From<&ControllerJoyStickEvent> for AbsoluteAxisCode {
  fn from(value: &ControllerJoyStickEvent) -> Self {
    match value.joystick() {
      JoyStick::Left => match value.axis() {
        JoyStickAxis::X => Self::ABS_X,
        JoyStickAxis::Y => Self::ABS_Y,
      },
      JoyStick::Right => match value.axis() {
        JoyStickAxis::X => Self::ABS_RX,
        JoyStickAxis::Y => Self::ABS_RY,
      },
    }
  }
}
