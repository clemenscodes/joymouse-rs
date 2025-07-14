use controller::{ControllerButton, StateError};
use evdev::KeyCode;

#[derive(Debug)]
pub enum ButtonError {
  UnsupportedKeyCode(KeyCode),
  InvalidState(StateError),
  InvalidButton(ControllerButton),
}

impl From<KeyCode> for ButtonError {
  fn from(v: KeyCode) -> Self {
    Self::UnsupportedKeyCode(v)
  }
}

impl From<StateError> for ButtonError {
  fn from(value: StateError) -> Self {
    Self::InvalidState(value)
  }
}

impl From<ControllerButton> for ButtonError {
  fn from(v: ControllerButton) -> Self {
    Self::InvalidButton(v)
  }
}

impl std::fmt::Display for ButtonError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      ButtonError::UnsupportedKeyCode(key_code) => {
        writeln!(f, "Unsupported key: {:#?}", key_code)
      }
      ButtonError::InvalidState(state) => {
        writeln!(f, "{}", state)
      }
      ButtonError::InvalidButton(controller_button) => {
        writeln!(f, "Invalid controller button: {:#?}", controller_button)
      }
    }
  }
}
