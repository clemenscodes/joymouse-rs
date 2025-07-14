use crate::{AxisError, ButtonError, PolarityError, StateError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JoyStickError {
  Axis(AxisError),
  InvalidState(StateError),
  InvalidPolarity(PolarityError),
  Button(ButtonError),
}

impl From<PolarityError> for JoyStickError {
  fn from(v: PolarityError) -> Self {
    Self::InvalidPolarity(v)
  }
}

impl From<AxisError> for JoyStickError {
  fn from(value: AxisError) -> Self {
    Self::Axis(value)
  }
}

impl From<StateError> for JoyStickError {
  fn from(value: StateError) -> Self {
    Self::InvalidState(value)
  }
}

impl From<ButtonError> for JoyStickError {
  fn from(value: ButtonError) -> Self {
    Self::Button(value)
  }
}

impl std::fmt::Display for JoyStickError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      JoyStickError::Axis(axis_error) => {
        writeln!(f, "{}", axis_error)
      }
      JoyStickError::Button(button_error) => {
        writeln!(f, "{}", button_error)
      }
      JoyStickError::InvalidState(state) => {
        writeln!(f, "{}", state)
      }
      JoyStickError::InvalidPolarity(polarity_error) => {
        writeln!(f, "{}", polarity_error)
      }
    }
  }
}

impl std::error::Error for JoyStickError {}
