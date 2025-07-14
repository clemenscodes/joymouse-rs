use crate::button::ControllerButton;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ButtonError {
  UnsupportedKeyCode(u16),
  InvalidButton(ControllerButton),
}

impl From<u16> for ButtonError {
  fn from(v: u16) -> Self {
    Self::UnsupportedKeyCode(v)
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
      ButtonError::InvalidButton(controller_button) => {
        writeln!(f, "Invalid controller button: {:#?}", controller_button)
      }
    }
  }
}
