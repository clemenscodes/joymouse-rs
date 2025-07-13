#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumericKeyError {
  Digit(u8),
  Key(char),
  Code(u16),
}

impl std::fmt::Display for NumericKeyError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Digit(n) => write!(f, "invalid numeric key: '{}'", n),
      Self::Key(key) => write!(f, "invalid numeric char: '{}'", key),
      Self::Code(code) => write!(f, "invalid numeric key code: '{}'", code),
    }
  }
}

impl std::error::Error for NumericKeyError {}
