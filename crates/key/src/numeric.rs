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
      other => Err(NumericKeyError::InvalidDigit(other)),
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
      _ => Err(NumericKeyError::InvalidKey(c)),
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
      Err(NumericKeyError::InvalidKey(value.chars().next().unwrap()))
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumericKeyError {
  InvalidDigit(u8),
  InvalidKey(char),
}

impl std::fmt::Display for NumericKeyError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::InvalidDigit(n) => write!(f, "invalid numeric key: '{}'", n),
      Self::InvalidKey(key) => write!(f, "invalid numeric char: '{}'", key),
    }
  }
}

impl std::error::Error for NumericKeyError {}

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
    assert!(matches!(NumericKey::try_from(10), Err(NumericKeyError::InvalidDigit(10))));
    assert!(matches!(NumericKey::try_from(255), Err(NumericKeyError::InvalidDigit(255))));
  }

  #[test]
  fn test_try_from_char_valid() {
    assert_eq!(NumericKey::try_from('0').unwrap(), NumericKey::Num0);
    assert_eq!(NumericKey::try_from('9').unwrap(), NumericKey::Num9);
  }

  #[test]
  fn test_try_from_char_invalid() {
    assert!(matches!(NumericKey::try_from('x'), Err(NumericKeyError::InvalidKey('x'))));
    assert!(matches!(NumericKey::try_from(' '), Err(NumericKeyError::InvalidKey(' '))));
  }

  #[test]
  fn test_try_from_str_valid() {
    assert_eq!(NumericKey::try_from("1").unwrap(), NumericKey::Num1);
    assert_eq!(NumericKey::try_from("9").unwrap(), NumericKey::Num9);
  }

  #[test]
  fn test_try_from_str_invalid() {
    assert!(matches!(NumericKey::try_from("x"), Err(NumericKeyError::InvalidKey('x'))));
    assert!(matches!(NumericKey::try_from("12"), Err(NumericKeyError::InvalidKey('1'))));
  }

  #[test]
  fn test_as_str_and_display() {
    assert_eq!(NumericKey::Num4.as_str(), "4");
    assert_eq!(NumericKey::Num4.to_string(), "4");

    assert_eq!(NumericKey::Num0.as_str(), "0");
    assert_eq!(NumericKey::Num0.to_string(), "0");
  }
}
