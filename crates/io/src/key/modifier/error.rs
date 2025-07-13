#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModifierKeyError {
  InvalidKey(String),
  InvalidCode(u16),
}

impl std::fmt::Display for ModifierKeyError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::InvalidKey(key) => write!(f, "invalid modifier key: '{}'", key),
      Self::InvalidCode(code) => write!(f, "invalid modifier code: '{}'", code),
    }
  }
}

impl std::error::Error for ModifierKeyError {}
