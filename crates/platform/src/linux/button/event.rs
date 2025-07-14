use controller::{ButtonError, ButtonEvent, State};
use evdev::{EventType, InputEvent, KeyEvent};

use crate::linux::button::{
  try_controller_button_from_keycode, try_key_code_from_controller_button,
};

pub fn try_from_key_event_for_button_event(event: KeyEvent) -> Result<ButtonEvent, ButtonError> {
  let code = event.code();
  let state = State::try_from(event.value())?;
  let button = try_controller_button_from_keycode(code)?;
  let event = ButtonEvent::new(button, state);
  Ok(event)
}

pub fn from_button_event_for_input_event(event: ButtonEvent) -> InputEvent {
  let code = try_key_code_from_controller_button(event.button()).unwrap();
  let value = event.state().as_value();
  InputEvent::new(EventType::KEY.0, code.code(), value)
}
