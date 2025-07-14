use controller::State;
use evdev::{EventType, InputEvent, KeyEvent};

use crate::button::{
  try_controller_button_from_keycode, try_key_code_from_controller_button, ButtonError,
  ControllerButton,
};

#[derive(Debug)]
pub struct ControllerButtonEvent {
  button: ControllerButton,
  state: State,
}

impl ControllerButtonEvent {
  pub fn button(&self) -> ControllerButton {
    self.button
  }

  pub fn state(&self) -> &State {
    &self.state
  }
}

impl TryFrom<KeyEvent> for ControllerButtonEvent {
  type Error = ButtonError;

  fn try_from(value: KeyEvent) -> Result<Self, Self::Error> {
    let code = value.code();
    let state = State::try_from(value.value())?;

    let button = try_controller_button_from_keycode(code)?;

    Ok(Self {
      button,
      state,
    })
  }
}

impl From<ControllerButtonEvent> for InputEvent {
  fn from(value: ControllerButtonEvent) -> Self {
    let code = try_key_code_from_controller_button(value.button()).unwrap();
    let value = value.state().as_value();
    Self::new(EventType::KEY.0, code.0, value)
  }
}
