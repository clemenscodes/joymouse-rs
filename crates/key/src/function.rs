use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FunctionKey {
  F1,
  F2,
  F3,
  F4,
  F5,
  F6,
  F7,
  F8,
  F9,
  F10,
  F11,
  F12,
}

impl FunctionKey {
  pub fn as_str(&self) -> &'static str {
    match self {
      FunctionKey::F1 => "f1",
      FunctionKey::F2 => "f2",
      FunctionKey::F3 => "f3",
      FunctionKey::F4 => "f4",
      FunctionKey::F5 => "f5",
      FunctionKey::F6 => "f6",
      FunctionKey::F7 => "f7",
      FunctionKey::F8 => "f8",
      FunctionKey::F9 => "f9",
      FunctionKey::F10 => "f10",
      FunctionKey::F11 => "f11",
      FunctionKey::F12 => "f12",
    }
  }
}

impl TryFrom<u8> for FunctionKey {
  type Error = FunctionKeyError;

  fn try_from(n: u8) -> Result<Self, Self::Error> {
    match n {
      1 => Ok(Self::F1),
      2 => Ok(Self::F2),
      3 => Ok(Self::F3),
      4 => Ok(Self::F4),
      5 => Ok(Self::F5),
      6 => Ok(Self::F6),
      7 => Ok(Self::F7),
      8 => Ok(Self::F8),
      9 => Ok(Self::F9),
      10 => Ok(Self::F10),
      11 => Ok(Self::F11),
      12 => Ok(Self::F12),
      other => Err(FunctionKeyError::Number(other)),
    }
  }
}

impl TryFrom<&str> for FunctionKey {
  type Error = FunctionKeyError;

  fn try_from(s: &str) -> Result<Self, Self::Error> {
    if let Some(num) = s.strip_prefix('f') {
      if let Ok(n) = num.parse::<u8>() {
        return Self::try_from(n);
      }
    }
    Err(FunctionKeyError::Format)
  }
}

#[cfg(not(windows))]
impl TryFrom<evdev::KeyCode> for FunctionKey {
  type Error = FunctionKeyError;

  fn try_from(code: evdev::KeyCode) -> Result<Self, Self::Error> {
    Ok(match code {
      evdev::KeyCode::KEY_F1 => Self::F1,
      evdev::KeyCode::KEY_F2 => Self::F2,
      evdev::KeyCode::KEY_F3 => Self::F3,
      evdev::KeyCode::KEY_F4 => Self::F4,
      evdev::KeyCode::KEY_F5 => Self::F5,
      evdev::KeyCode::KEY_F6 => Self::F6,
      evdev::KeyCode::KEY_F7 => Self::F7,
      evdev::KeyCode::KEY_F8 => Self::F8,
      evdev::KeyCode::KEY_F9 => Self::F9,
      evdev::KeyCode::KEY_F10 => Self::F10,
      evdev::KeyCode::KEY_F11 => Self::F11,
      evdev::KeyCode::KEY_F12 => Self::F12,
      _ => return Err(FunctionKeyError::Code(code.code())),
    })
  }
}

impl std::fmt::Display for FunctionKey {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.as_str())
  }
}

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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_valid_lowercase() {
    assert_eq!(FunctionKey::try_from("f1").unwrap(), FunctionKey::F1);
    assert_eq!(FunctionKey::try_from("f12").unwrap(), FunctionKey::F12);
  }

  #[test]
  fn test_invalid_prefix() {
    assert!(FunctionKey::try_from("g1").is_err());
    assert!(FunctionKey::try_from("z9").is_err());
    assert!(FunctionKey::try_from("12").is_err());
    assert!(FunctionKey::try_from("fn1").is_err());
  }

  #[test]
  fn test_out_of_range() {
    match FunctionKey::try_from("f0") {
      Err(FunctionKeyError::Number(0)) => (),
      _ => panic!("Expected InvalidNumber(0)"),
    }

    match FunctionKey::try_from("f13") {
      Err(FunctionKeyError::Number(13)) => (),
      _ => panic!("Expected Number(13)"),
    }
  }

  #[test]
  fn test_as_str() {
    assert_eq!(FunctionKey::F5.as_str(), "f5");
    assert_eq!(FunctionKey::F12.as_str(), "f12");
  }

  #[test]
  fn test_display() {
    assert_eq!(FunctionKey::F3.to_string(), "f3");
    assert_eq!(FunctionKey::F11.to_string(), "f11");
  }

  #[cfg(not(windows))]
  mod keycode_tests {
    use super::*;
    use evdev::KeyCode;

    #[test]
    fn test_valid_function_keycodes() {
      assert_eq!(FunctionKey::try_from(KeyCode::KEY_F1).unwrap(), FunctionKey::F1);
      assert_eq!(FunctionKey::try_from(KeyCode::KEY_F12).unwrap(), FunctionKey::F12);
    }

    #[test]
    fn test_invalid_function_keycode() {
      let result = FunctionKey::try_from(KeyCode::KEY_A);
      assert!(matches!(
        result,
        Err(FunctionKeyError::Code(code)) if code == KeyCode::KEY_A.code()
      ));
    }
  }
}
