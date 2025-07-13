use evdev::{KeyCode, RelativeAxisCode};

use crate::{
  button::ButtonError,
  joystick::{polarity::PolarityError, AxisError},
  state::StateError,
};

#[derive(Debug)]
pub enum JoyStickError {
  Axis(AxisError),
  UnsupportedAxisCode(RelativeAxisCode),
  UnsupportedKeyCode(KeyCode),
  InvalidState(StateError),
  InvalidPolarity(PolarityError),
  Button(ButtonError),
}

impl From<PolarityError> for JoyStickError {
  fn from(v: PolarityError) -> Self {
    Self::InvalidPolarity(v)
  }
}

impl From<AxisError> for JoyStickError {
  fn from(value: AxisError) -> Self {
    Self::Axis(value)
  }
}

impl From<RelativeAxisCode> for JoyStickError {
  fn from(v: RelativeAxisCode) -> Self {
    Self::UnsupportedAxisCode(v)
  }
}

impl From<KeyCode> for JoyStickError {
  fn from(v: KeyCode) -> Self {
    Self::UnsupportedKeyCode(v)
  }
}

impl From<StateError> for JoyStickError {
  fn from(value: StateError) -> Self {
    Self::InvalidState(value)
  }
}

impl From<ButtonError> for JoyStickError {
  fn from(value: ButtonError) -> Self {
    Self::Button(value)
  }
}

impl std::fmt::Display for JoyStickError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      JoyStickError::Axis(axis_error) => {
        writeln!(f, "{}", axis_error)
      }
      JoyStickError::UnsupportedAxisCode(relative_axis_code) => {
        writeln!(f, "Joystick unsupported axis code: {:#?}", relative_axis_code)
      }
      JoyStickError::UnsupportedKeyCode(key_code) => {
        writeln!(f, "Joystick unsupported key code: {:#?}", key_code)
      }
      JoyStickError::Button(button_error) => {
        writeln!(f, "{}", button_error)
      }
      JoyStickError::InvalidState(state) => {
        writeln!(f, "{}", state)
      }
      JoyStickError::InvalidPolarity(polarity_error) => {
        writeln!(f, "{}", polarity_error)
      }
    }
  }
}
