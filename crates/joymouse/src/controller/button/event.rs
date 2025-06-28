use evdev::KeyEvent;

use crate::controller::{
  button::{ButtonError, ControllerButton},
  state::State,
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
    let state = match value.value() {
      0 => State::Released,
      1 => State::Pressed,
      2 => State::Held,
      other => return Err(ButtonError::InvalidState(other)),
    };

    let button = ControllerButton::try_from(code)?;

    Ok(Self {
      button,
      state,
    })
  }
}
