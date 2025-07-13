#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArrowKeyError {
  InvalidKey(String),
  InvalidCode(u16),
}

impl std::fmt::Display for ArrowKeyError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::InvalidKey(key) => write!(f, "invalid arrow key: '{}'", key),
      Self::InvalidCode(code) => write!(f, "invalid arrow code: '{}'", code),
    }
  }
}

impl std::error::Error for ArrowKeyError {}
