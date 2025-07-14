use crate::{ButtonError, JoyStickError};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ControllerError {
  Button(ButtonError),
  Joystick(JoyStickError),
}

impl std::fmt::Display for ControllerError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      ControllerError::Button(button_error) => write!(f, "{}", button_error),
      ControllerError::Joystick(joy_stick_error) => write!(f, "{}", joy_stick_error),
    }
  }
}

impl std::error::Error for ControllerError {}
