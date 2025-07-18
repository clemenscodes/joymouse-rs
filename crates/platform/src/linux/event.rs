use bindings::JOYSTICK_KEYS;
use controller::{ControllerError, ControllerEvent};
use io::Key;

use evdev::{EventSummary, InputEvent, KeyEvent, RelativeAxisEvent};

use crate::linux::{
  button::{from_button_event_for_input_event, try_from_key_event_for_button_event},
  joystick::{
    from_joystick_event_for_input_event, try_from_key_event_for_joystick_event,
    try_from_relative_axis_event_for_joystick_event,
  },
};

pub fn from_controller_event_for_input_event(event: ControllerEvent) -> InputEvent {
  match event {
    ControllerEvent::Button(button_event) => from_button_event_for_input_event(button_event),
    ControllerEvent::JoyStick(joystick_event) => {
      from_joystick_event_for_input_event(joystick_event)
    }
  }
}

pub fn try_from_key_event_for_controller_event(
  event: KeyEvent,
) -> Result<ControllerEvent, ControllerError> {
  let code = event.code();
  let key = Key::try_from(code).map_err(|_| ControllerError::UnsupportedEvent)?;

  if JOYSTICK_KEYS.key_is_joystick_key(key) {
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
