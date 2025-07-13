#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SystemKeyError {
  InvalidKey(String),
  InvalidCode(u16),
}

impl std::fmt::Display for SystemKeyError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::InvalidKey(s) => write!(f, "invalid system key: '{}'", s),
      Self::InvalidCode(code) => write!(f, "invalid system key code: '{}'", code),
    }
  }
}

impl std::error::Error for SystemKeyError {}
