#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MouseKeyError {
  InvalidKey(String),
  InvalidCode(u16),
}

impl std::fmt::Display for MouseKeyError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::InvalidKey(s) => write!(f, "invalid mouse key: '{}'", s),
      Self::InvalidCode(code) => write!(f, "invalid mouse key code: '{}'", code),
    }
  }
}

impl std::error::Error for MouseKeyError {}
