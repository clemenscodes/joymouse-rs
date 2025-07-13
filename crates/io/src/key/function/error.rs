#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FunctionKeyError {
  Number(u8),
  Format,
  Code(u16),
}

impl std::fmt::Display for FunctionKeyError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Number(n) => write!(f, "invalid function key: f{}", n),
      Self::Format => write!(f, "invalid function key format"),
      Self::Code(code) => write!(f, "invalid function key code: '{}'", code),
    }
  }
}

impl std::error::Error for FunctionKeyError {}
