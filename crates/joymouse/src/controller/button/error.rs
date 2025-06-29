use evdev::KeyCode;

use crate::controller::button::ControllerButton;

#[derive(Debug)]
pub enum ButtonError {
  UnsupportedKeyCode(KeyCode),
  InvalidState(i32),
  InvalidButton(ControllerButton),
}

impl std::fmt::Display for ButtonError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      ButtonError::UnsupportedKeyCode(key_code) => {
        writeln!(f, "Unsupported key: {:#?}", key_code)
      }
      ButtonError::InvalidState(state) => {
        writeln!(f, "Invalid state: {}", state)
      }
      ButtonError::InvalidButton(controller_button) => {
        writeln!(f, "Invalid controller button: {:#?}", controller_button)
      }
    }
  }
}
