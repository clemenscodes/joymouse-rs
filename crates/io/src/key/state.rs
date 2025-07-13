#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum KeyState {
  Pressed,
  Released,
  Held,
}

impl Default for KeyState {
  fn default() -> Self {
    Self::Released
  }
}

impl KeyState {
  pub fn as_value(&self) -> i32 {
    match self {
      Self::Released => 0,
      Self::Pressed => 1,
      Self::Held => 2,
    }
  }
}

impl TryFrom<i32> for KeyState {
  type Error = KeyStateError;

  fn try_from(value: i32) -> Result<Self, Self::Error> {
    let state = match value {
      0 => Self::Released,
      1 => Self::Pressed,
      2 => Self::Held,
      other => return Err(KeyStateError::State(other)),
    };
    Ok(state)
  }
}

#[derive(Debug)]
pub enum KeyStateError {
  State(i32),
}

impl std::fmt::Display for KeyStateError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      KeyStateError::State(state) => {
        writeln!(f, "Invalid state: {}", state)
      }
    }
  }
}
