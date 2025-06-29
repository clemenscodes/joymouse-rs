use evdev::{EventSummary, KeyEvent, RelativeAxisEvent};

use crate::controller::{
  button::ControllerButtonEvent,
  error::ControllerError,
  joystick::{ControllerJoyStickEvent, JOYSTICK_KEYS},
};

#[derive(Debug)]
pub enum ControllerEvent {
  Button(ControllerButtonEvent),
  JoyStick(ControllerJoyStickEvent),
}

impl From<ControllerButtonEvent> for ControllerEvent {
  fn from(event: ControllerButtonEvent) -> Self {
    Self::Button(event)
  }
}

impl From<ControllerJoyStickEvent> for ControllerEvent {
  fn from(event: ControllerJoyStickEvent) -> Self {
    Self::JoyStick(event)
  }
}

impl TryFrom<KeyEvent> for ControllerEvent {
  type Error = ControllerError;

  fn try_from(value: KeyEvent) -> Result<Self, Self::Error> {
    let code = value.code();

    if JOYSTICK_KEYS.code_is_joystick_key(code) {
      let event = Self::from(ControllerJoyStickEvent::try_from(value)?);
      return Ok(event);
    }

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
