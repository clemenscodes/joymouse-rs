use evdev::{KeyCode, RelativeAxisCode};

use crate::controller::{button::ButtonError, joystick::AxisError};

#[derive(Debug)]
pub enum JoyStickError {
  Axis(AxisError),
  UnsupportedAxisCode(RelativeAxisCode),
  UnsupportedKeyCode(KeyCode),
  Button(ButtonError),
}

impl From<AxisError> for JoyStickError {
  fn from(value: AxisError) -> Self {
    Self::Axis(value)
  }
}

impl std::fmt::Display for JoyStickError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      JoyStickError::Axis(axis_error) => {
        writeln!(f, "Joystick axis error: {}", axis_error)
      }
      JoyStickError::UnsupportedAxisCode(relative_axis_code) => {
        writeln!(f, "Joystick unsupported axis code: {:#?}", relative_axis_code)
      }
      JoyStickError::UnsupportedKeyCode(key_code) => {
        writeln!(f, "Joystick unsupported key code: {:#?}", key_code)
      }
      JoyStickError::Button(button_error) => {
        writeln!(f, "Joystick button error: {}", button_error)
      }
    }
  }
}
