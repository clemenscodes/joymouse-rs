use crate::ButtonError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AxisError {
  Button(ButtonError),
  Unknown,
}

impl From<ButtonError> for AxisError {
  fn from(value: ButtonError) -> Self {
    Self::Button(value)
  }
}

impl std::fmt::Display for AxisError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      AxisError::Button(button_error) => {
        writeln!(f, "Axis button error: {}", button_error)
      }
      AxisError::Unknown => {
        writeln!(f, "Axis unknown error")
      }
    }
  }
}

impl std::error::Error for AxisError {}
