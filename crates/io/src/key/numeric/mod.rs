mod error;

pub use error::NumericKeyError;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NumericKey {
  Num0,
  Num1,
  Num2,
  Num3,
  Num4,
  Num5,
  Num6,
  Num7,
  Num8,
  Num9,
}

impl NumericKey {
  pub fn as_str(&self) -> &'static str {
    match self {
      NumericKey::Num0 => "0",
      NumericKey::Num1 => "1",
      NumericKey::Num2 => "2",
      NumericKey::Num3 => "3",
      NumericKey::Num4 => "4",
      NumericKey::Num5 => "5",
      NumericKey::Num6 => "6",
      NumericKey::Num7 => "7",
      NumericKey::Num8 => "8",
      NumericKey::Num9 => "9",
    }
  }
}

impl std::fmt::Display for NumericKey {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.as_str())
  }
}

impl TryFrom<u8> for NumericKey {
  type Error = NumericKeyError;

  fn try_from(n: u8) -> Result<Self, Self::Error> {
    match n {
      0 => Ok(Self::Num0),
      1 => Ok(Self::Num1),
      2 => Ok(Self::Num2),
      3 => Ok(Self::Num3),
      4 => Ok(Self::Num4),
      5 => Ok(Self::Num5),
      6 => Ok(Self::Num6),
      7 => Ok(Self::Num7),
      8 => Ok(Self::Num8),
      9 => Ok(Self::Num9),
      other => Err(NumericKeyError::Digit(other)),
    }
  }
}

impl TryFrom<char> for NumericKey {
  type Error = NumericKeyError;

  fn try_from(c: char) -> Result<Self, Self::Error> {
    match c {
      '0' => Ok(Self::Num0),
      '1' => Ok(Self::Num1),
      '2' => Ok(Self::Num2),
      '3' => Ok(Self::Num3),
      '4' => Ok(Self::Num4),
      '5' => Ok(Self::Num5),
      '6' => Ok(Self::Num6),
      '7' => Ok(Self::Num7),
      '8' => Ok(Self::Num8),
      '9' => Ok(Self::Num9),
      _ => Err(NumericKeyError::Key(c)),
    }
  }
}

impl TryFrom<&str> for NumericKey {
  type Error = NumericKeyError;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    if value.len() == 1 {
      let ch = value.chars().next().unwrap();
      Self::try_from(ch)
    } else {
      Err(NumericKeyError::Key(value.chars().next().unwrap()))
    }
  }
}

#[cfg(not(windows))]
impl TryFrom<evdev::KeyCode> for NumericKey {
  type Error = NumericKeyError;

  fn try_from(code: evdev::KeyCode) -> Result<Self, Self::Error> {
    Ok(match code {
      evdev::KeyCode::KEY_0 => Self::Num0,
      evdev::KeyCode::KEY_1 => Self::Num1,
      evdev::KeyCode::KEY_2 => Self::Num2,
      evdev::KeyCode::KEY_3 => Self::Num3,
      evdev::KeyCode::KEY_4 => Self::Num4,
      evdev::KeyCode::KEY_5 => Self::Num5,
      evdev::KeyCode::KEY_6 => Self::Num6,
      evdev::KeyCode::KEY_7 => Self::Num7,
      evdev::KeyCode::KEY_8 => Self::Num8,
      evdev::KeyCode::KEY_9 => Self::Num9,
      _ => return Err(NumericKeyError::Code(code.code())),
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_try_from_u8_valid() {
    for n in 0..=9 {
      let key = NumericKey::try_from(n).unwrap();
      assert_eq!(key.as_str(), &n.to_string());
    }
  }

  #[test]
  fn test_try_from_u8_invalid() {
    assert!(matches!(NumericKey::try_from(10), Err(NumericKeyError::Digit(10))));
    assert!(matches!(NumericKey::try_from(255), Err(NumericKeyError::Digit(255))));
  }

  #[test]
  fn test_try_from_char_valid() {
    assert_eq!(NumericKey::try_from('0').unwrap(), NumericKey::Num0);
    assert_eq!(NumericKey::try_from('9').unwrap(), NumericKey::Num9);
  }

  #[test]
  fn test_try_from_char_invalid() {
    assert!(matches!(NumericKey::try_from('x'), Err(NumericKeyError::Key('x'))));
    assert!(matches!(NumericKey::try_from(' '), Err(NumericKeyError::Key(' '))));
  }

  #[test]
  fn test_try_from_str_valid() {
    assert_eq!(NumericKey::try_from("1").unwrap(), NumericKey::Num1);
    assert_eq!(NumericKey::try_from("9").unwrap(), NumericKey::Num9);
  }

  #[test]
  fn test_try_from_str_invalid() {
    assert!(matches!(NumericKey::try_from("x"), Err(NumericKeyError::Key('x'))));
    assert!(matches!(NumericKey::try_from("12"), Err(NumericKeyError::Key('1'))));
  }

  #[test]
  fn test_as_str_and_display() {
    assert_eq!(NumericKey::Num4.as_str(), "4");
    assert_eq!(NumericKey::Num4.to_string(), "4");

    assert_eq!(NumericKey::Num0.as_str(), "0");
    assert_eq!(NumericKey::Num0.to_string(), "0");
  }

  #[cfg(not(windows))]
  mod keycode_tests {
    use super::*;

    #[test]
    fn test_numeric_key_from_keycode_valid() {
      assert_eq!(NumericKey::try_from(evdev::KeyCode::KEY_0).unwrap(), NumericKey::Num0);
      assert_eq!(NumericKey::try_from(evdev::KeyCode::KEY_3).unwrap(), NumericKey::Num3);
      assert_eq!(NumericKey::try_from(evdev::KeyCode::KEY_9).unwrap(), NumericKey::Num9);
    }

    #[test]
    fn test_numeric_key_from_keycode_invalid() {
      let err = NumericKey::try_from(evdev::KeyCode::KEY_A).unwrap_err();
      assert!(matches!(err, NumericKeyError::Code(code) if code == evdev::KeyCode::KEY_A.code()));
    }
  }
}
