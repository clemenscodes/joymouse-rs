mod error;

pub use error::ArrowKeyError;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ArrowKey {
  Up,
  Down,
  Left,
  Right,
}

impl ArrowKey {
  pub fn as_str(&self) -> &'static str {
    match self {
      ArrowKey::Up => "up",
      ArrowKey::Down => "down",
      ArrowKey::Left => "left",
      ArrowKey::Right => "right",
    }
  }
}

impl TryFrom<&str> for ArrowKey {
  type Error = ArrowKeyError;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    match value.to_ascii_lowercase().as_str() {
      "up" => Ok(Self::Up),
      "down" => Ok(Self::Down),
      "left" => Ok(Self::Left),
      "right" => Ok(Self::Right),
      _ => Err(ArrowKeyError::InvalidKey(value.to_owned())),
    }
  }
}

#[cfg(not(windows))]
impl TryFrom<evdev::KeyCode> for ArrowKey {
  type Error = ArrowKeyError;

  fn try_from(code: evdev::KeyCode) -> Result<Self, Self::Error> {
    Ok(match code {
      evdev::KeyCode::KEY_UP => Self::Up,
      evdev::KeyCode::KEY_DOWN => Self::Down,
      evdev::KeyCode::KEY_LEFT => Self::Left,
      evdev::KeyCode::KEY_RIGHT => Self::Right,
      _ => return Err(ArrowKeyError::InvalidCode(code.code())),
    })
  }
}

impl std::fmt::Display for ArrowKey {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.as_str())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_valid_inputs_lowercase() {
    assert_eq!(ArrowKey::try_from("up").unwrap(), ArrowKey::Up);
    assert_eq!(ArrowKey::try_from("down").unwrap(), ArrowKey::Down);
    assert_eq!(ArrowKey::try_from("left").unwrap(), ArrowKey::Left);
    assert_eq!(ArrowKey::try_from("right").unwrap(), ArrowKey::Right);
  }

  #[test]
  fn test_valid_inputs_uppercase() {
    assert_eq!(ArrowKey::try_from("UP").unwrap(), ArrowKey::Up);
    assert_eq!(ArrowKey::try_from("DOWN").unwrap(), ArrowKey::Down);
    assert_eq!(ArrowKey::try_from("LEFT").unwrap(), ArrowKey::Left);
    assert_eq!(ArrowKey::try_from("RIGHT").unwrap(), ArrowKey::Right);
  }

  #[test]
  fn test_invalid_input() {
    let err = ArrowKey::try_from("center").unwrap_err();
    assert!(matches!(err, ArrowKeyError::InvalidKey(s) if s == "center"));

    let err = ArrowKey::try_from("").unwrap_err();
    assert!(matches!(err, ArrowKeyError::InvalidKey(s) if s.is_empty()));
  }

  #[test]
  fn test_as_str() {
    assert_eq!(ArrowKey::Up.as_str(), "up");
    assert_eq!(ArrowKey::Down.as_str(), "down");
    assert_eq!(ArrowKey::Left.as_str(), "left");
    assert_eq!(ArrowKey::Right.as_str(), "right");
  }

  #[test]
  fn test_display() {
    assert_eq!(ArrowKey::Left.to_string(), "left");
    assert_eq!(ArrowKey::Right.to_string(), "right");
  }

  #[cfg(not(windows))]
  #[test]
  fn test_valid_keycode_conversion() {
    assert_eq!(ArrowKey::try_from(evdev::KeyCode::KEY_UP).unwrap(), ArrowKey::Up);
  }

  #[cfg(not(windows))]
  #[test]
  fn test_invalid_keycode_conversion() {
    let result = ArrowKey::try_from(evdev::KeyCode::KEY_A);
    assert!(matches!(result, Err(ArrowKeyError::InvalidCode(_))));
  }
}
