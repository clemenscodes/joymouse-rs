use crate::{AxisError, ButtonError, JoyStickError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ControllerError {
  Button(ButtonError),
  JoyStick(JoyStickError),
  UnsupportedEvent,
}

impl From<AxisError> for ControllerError {
  fn from(value: AxisError) -> Self {
    Self::from(JoyStickError::from(value))
  }
}

impl From<ButtonError> for ControllerError {
  fn from(value: ButtonError) -> Self {
    Self::Button(value)
  }
}

impl From<JoyStickError> for ControllerError {
  fn from(v: JoyStickError) -> Self {
    Self::JoyStick(v)
  }
}

impl std::fmt::Display for ControllerError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      ControllerError::Button(button_error) => {
        writeln!(f, "Controller button error: {}", button_error)
      }
      ControllerError::JoyStick(joy_stick_error) => {
        writeln!(f, "Controller joystick error: {}", joy_stick_error)
      }
      ControllerError::UnsupportedEvent => {
        writeln!(f, "Controller unsupported event")
      }
    }
  }
}

impl std::error::Error for ControllerError {}
