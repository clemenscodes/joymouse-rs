use controller::{ControllerError, ControllerEvent};
use evdev::{EventSummary, KeyEvent, RelativeAxisEvent};

use crate::{
  button::try_from_key_event_for_button_event,
  joystick::{
    try_from_key_event_for_joystick_event, try_from_relative_axis_event_for_joystick_event,
    JOYSTICK_KEYS,
  },
};

pub fn try_from_key_event_for_controller_event(
  event: KeyEvent,
) -> Result<ControllerEvent, ControllerError> {
  let code = event.code();

  if JOYSTICK_KEYS.code_is_joystick_key(code) {
    let joystick_event = try_from_key_event_for_joystick_event(event)?;
    let event = ControllerEvent::from(joystick_event);
    return Ok(event);
  }

  let button_event = try_from_key_event_for_button_event(event)?;
  let event = ControllerEvent::from(button_event);
  Ok(event)
}

pub fn try_from_relative_axis_event_for_controller_event(
  event: RelativeAxisEvent,
) -> Result<ControllerEvent, ControllerError> {
  let joystick_event = try_from_relative_axis_event_for_joystick_event(event)?;
  let event = ControllerEvent::from(joystick_event);
  Ok(event)
}

pub fn try_from_event_summary_for_controller_event(
  event: EventSummary,
) -> Result<ControllerEvent, ControllerError> {
  let event = match event {
    EventSummary::Key(event, _, _) => try_from_key_event_for_controller_event(event)?,
    EventSummary::RelativeAxis(event, _, _) => {
      try_from_relative_axis_event_for_controller_event(event)?
    }
    _ => return Err(ControllerError::UnsupportedEvent),
  };

  Ok(event)
}
