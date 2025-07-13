use crate::{
  AlphabeticKeyError, ArrowKeyError, FunctionKeyError, ModifierKeyError, MouseKeyError,
  NumericKeyError, SystemKeyError,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyError {
  Alphabetic(AlphabeticKeyError),
  Numeric(NumericKeyError),
  Function(FunctionKeyError),
  Arrow(ArrowKeyError),
  Modifier(ModifierKeyError),
  System(SystemKeyError),
  Mouse(MouseKeyError),
}

impl From<MouseKeyError> for KeyError {
  fn from(v: MouseKeyError) -> Self {
    Self::Mouse(v)
  }
}

impl From<SystemKeyError> for KeyError {
  fn from(v: SystemKeyError) -> Self {
    Self::System(v)
  }
}

impl From<ModifierKeyError> for KeyError {
  fn from(v: ModifierKeyError) -> Self {
    Self::Modifier(v)
  }
}

impl From<ArrowKeyError> for KeyError {
  fn from(v: ArrowKeyError) -> Self {
    Self::Arrow(v)
  }
}

impl From<FunctionKeyError> for KeyError {
  fn from(v: FunctionKeyError) -> Self {
    Self::Function(v)
  }
}

impl From<NumericKeyError> for KeyError {
  fn from(v: NumericKeyError) -> Self {
    Self::Numeric(v)
  }
}

impl From<AlphabeticKeyError> for KeyError {
  fn from(v: AlphabeticKeyError) -> Self {
    Self::Alphabetic(v)
  }
}

impl std::fmt::Display for KeyError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Alphabetic(e) => write!(f, "alphabetic key error: {e}"),
      Self::Numeric(e) => write!(f, "numeric key error: {e}"),
      Self::Function(e) => write!(f, "function key error: {e}"),
      Self::Arrow(e) => write!(f, "arrow key error: {e}"),
      Self::Modifier(e) => write!(f, "modifier key error: {e}"),
      Self::System(e) => write!(f, "system key error: {e}"),
      Self::Mouse(e) => write!(f, "mouse key error: {e}"),
    }
  }
}

impl std::error::Error for KeyError {}
