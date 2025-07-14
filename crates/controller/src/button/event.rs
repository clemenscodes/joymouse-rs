use crate::{ControllerButton, State};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ButtonEvent {
  button: ControllerButton,
  state: State,
}

impl ButtonEvent {
  pub fn button(&self) -> ControllerButton {
    self.button
  }

  pub fn state(&self) -> &State {
    &self.state
  }
}
