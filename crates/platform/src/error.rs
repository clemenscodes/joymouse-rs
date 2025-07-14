use evdev::EventSummary;
use io::AxisError;

use crate::{button::ButtonError, joystick::JoyStickError};

#[derive(Debug)]
pub enum ControllerError {
  Button(ButtonError),
  JoyStick(JoyStickError),
  UnsupportedEvent(EventSummary),
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

impl From<EventSummary> for ControllerError {
  fn from(v: EventSummary) -> Self {
    Self::UnsupportedEvent(v)
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
      ControllerError::UnsupportedEvent(event_summary) => {
        writeln!(f, "Controller unsupported event: {:#?}", event_summary)
      }
    }
  }
}
