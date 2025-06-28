use evdev::{EventSummary, KeyEvent, RelativeAxisEvent};

use crate::controller::{
  button::{ButtonError, ControllerButtonEvent},
  joystick::{ControllerJoyStickEvent, JoyStickError},
};

#[derive(Debug)]
pub enum ControllerError {
  Button(ButtonError),
  JoyStick(JoyStickError),
  UnsupportedEvent(EventSummary),
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

#[derive(Debug)]
pub enum ControllerEvent {
  Button {
    event: ControllerButtonEvent,
  },
  JoyStick {
    event: ControllerJoyStickEvent,
  },
}

impl From<ControllerButtonEvent> for ControllerEvent {
  fn from(event: ControllerButtonEvent) -> Self {
    Self::Button {
      event,
    }
  }
}

impl From<ControllerJoyStickEvent> for ControllerEvent {
  fn from(event: ControllerJoyStickEvent) -> Self {
    Self::JoyStick {
      event,
    }
  }
}

impl TryFrom<KeyEvent> for ControllerEvent {
  type Error = ControllerError;

  fn try_from(value: KeyEvent) -> Result<Self, Self::Error> {
    let event = Self::from(ControllerButtonEvent::try_from(value)?);
    Ok(event)
  }
}

impl TryFrom<RelativeAxisEvent> for ControllerEvent {
  type Error = ControllerError;

  fn try_from(value: RelativeAxisEvent) -> Result<Self, Self::Error> {
    let event = Self::from(ControllerJoyStickEvent::try_from(value)?);
    Ok(event)
  }
}

impl TryFrom<EventSummary> for ControllerEvent {
  type Error = ControllerError;

  fn try_from(value: EventSummary) -> Result<Self, Self::Error> {
    let event = match value {
      EventSummary::Key(event, _, _) => Self::try_from(event)?,
      EventSummary::RelativeAxis(event, _, _) => Self::try_from(event)?,
      other => return Err(ControllerError::UnsupportedEvent(other)),
    };

    Ok(event)
  }
}
