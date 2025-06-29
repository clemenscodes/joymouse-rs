#[derive(Debug)]
pub enum State {
  Pressed,
  Released,
  Held,
}

impl State {
  pub fn as_value(&self) -> i32 {
    match self {
      State::Released => 0,
      State::Pressed => 1,
      State::Held => 2,
    }
  }
}

impl TryFrom<i32> for State {
  type Error = StateError;

  fn try_from(value: i32) -> Result<Self, Self::Error> {
    let state = match value {
      0 => Self::Released,
      1 => Self::Pressed,
      2 => Self::Held,
      other => return Err(StateError::InvalidState(other)),
    };
    Ok(state)
  }
}

#[derive(Debug)]
pub enum StateError {
  InvalidState(i32),
}

impl std::fmt::Display for StateError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      StateError::InvalidState(state) => {
        writeln!(f, "Invalid state: {}", state)
      }
    }
  }
}
