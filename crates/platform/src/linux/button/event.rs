use controller::{ButtonError, ButtonEvent, State};
use evdev::{EventType, InputEvent, KeyEvent};

use crate::linux::button::{
  try_from_controller_button_for_keycode, try_from_keycode_for_controller_button,
};

pub fn from_button_event_for_input_event(event: ButtonEvent) -> InputEvent {
  let value = (*event.state()).into();
  let code = try_from_controller_button_for_keycode(event.button()).unwrap();
  InputEvent::new(EventType::KEY.0, code.code(), value)
}

pub fn try_from_key_event_for_button_event(event: KeyEvent) -> Result<ButtonEvent, ButtonError> {
  let code = event.code();
  let state = State::try_from(event.value())?;
  let button = try_from_keycode_for_controller_button(code)?;
  let event = ButtonEvent::new(button, state);
  Ok(event)
}
