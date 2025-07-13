#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlphabeticKeyError {
  InvalidKey(char),
  InvalidCode(u16),
}

impl std::fmt::Display for AlphabeticKeyError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::InvalidKey(key) => write!(f, "invalid alphabetic key: '{}'", key),
      Self::InvalidCode(code) => write!(f, "invalid alphabetic code: '{}'", code),
    }
  }
}

impl std::error::Error for AlphabeticKeyError {}
