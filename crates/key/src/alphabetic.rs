use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AlphabeticKey {
  A,
  B,
  C,
  D,
  E,
  F,
  G,
  H,
  I,
  J,
  K,
  L,
  M,
  N,
  O,
  P,
  Q,
  R,
  S,
  T,
  U,
  V,
  W,
  X,
  Y,
  Z,
}

impl AlphabeticKey {
  pub fn as_str(&self) -> &'static str {
    match self {
      AlphabeticKey::A => "a",
      AlphabeticKey::B => "b",
      AlphabeticKey::C => "c",
      AlphabeticKey::D => "d",
      AlphabeticKey::E => "e",
      AlphabeticKey::F => "f",
      AlphabeticKey::G => "g",
      AlphabeticKey::H => "h",
      AlphabeticKey::I => "i",
      AlphabeticKey::J => "j",
      AlphabeticKey::K => "k",
      AlphabeticKey::L => "l",
      AlphabeticKey::M => "m",
      AlphabeticKey::N => "n",
      AlphabeticKey::O => "o",
      AlphabeticKey::P => "p",
      AlphabeticKey::Q => "q",
      AlphabeticKey::R => "r",
      AlphabeticKey::S => "s",
      AlphabeticKey::T => "t",
      AlphabeticKey::U => "u",
      AlphabeticKey::V => "v",
      AlphabeticKey::W => "w",
      AlphabeticKey::X => "x",
      AlphabeticKey::Y => "y",
      AlphabeticKey::Z => "z",
    }
  }
}

impl std::fmt::Display for AlphabeticKey {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.as_str())
  }
}

impl TryFrom<char> for AlphabeticKey {
  type Error = AlphabeticKeyError;

  fn try_from(c: char) -> Result<Self, Self::Error> {
    match c.to_ascii_lowercase() {
      'a' => Ok(Self::A),
      'b' => Ok(Self::B),
      'c' => Ok(Self::C),
      'd' => Ok(Self::D),
      'e' => Ok(Self::E),
      'f' => Ok(Self::F),
      'g' => Ok(Self::G),
      'h' => Ok(Self::H),
      'i' => Ok(Self::I),
      'j' => Ok(Self::J),
      'k' => Ok(Self::K),
      'l' => Ok(Self::L),
      'm' => Ok(Self::M),
      'n' => Ok(Self::N),
      'o' => Ok(Self::O),
      'p' => Ok(Self::P),
      'q' => Ok(Self::Q),
      'r' => Ok(Self::R),
      's' => Ok(Self::S),
      't' => Ok(Self::T),
      'u' => Ok(Self::U),
      'v' => Ok(Self::V),
      'w' => Ok(Self::W),
      'x' => Ok(Self::X),
      'y' => Ok(Self::Y),
      'z' => Ok(Self::Z),
      other => Err(AlphabeticKeyError::InvalidKey(other)),
    }
  }
}

impl TryFrom<&str> for AlphabeticKey {
  type Error = AlphabeticKeyError;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    if value.len() == 1 {
      let ch = value.chars().next().unwrap();
      Self::try_from(ch)
    } else {
      Err(AlphabeticKeyError::InvalidKey(value.chars().next().unwrap()))
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlphabeticKeyError {
  InvalidKey(char),
}

impl std::fmt::Display for AlphabeticKeyError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::InvalidKey(c) => write!(f, "invalid alphabetic key: '{}'", c),
    }
  }
}

impl std::error::Error for AlphabeticKeyError {}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_try_from_lowercase_char() {
    assert_eq!(AlphabeticKey::try_from('a').unwrap(), AlphabeticKey::A);
    assert_eq!(AlphabeticKey::try_from('z').unwrap(), AlphabeticKey::Z);
  }

  #[test]
  fn test_try_from_uppercase_char() {
    assert_eq!(AlphabeticKey::try_from('A').unwrap(), AlphabeticKey::A);
    assert_eq!(AlphabeticKey::try_from('Z').unwrap(), AlphabeticKey::Z);
  }

  #[test]
  fn test_try_from_invalid_char() {
    assert!(matches!(AlphabeticKey::try_from('1'), Err(AlphabeticKeyError::InvalidKey('1'))));

    assert!(matches!(AlphabeticKey::try_from('$'), Err(AlphabeticKeyError::InvalidKey('$'))));
  }

  #[test]
  fn test_try_from_valid_str() {
    assert_eq!(AlphabeticKey::try_from("a").unwrap(), AlphabeticKey::A);
    assert_eq!(AlphabeticKey::try_from("Z").unwrap(), AlphabeticKey::Z);
  }

  #[test]
  fn test_try_from_invalid_str() {
    assert!(AlphabeticKey::try_from("aa").is_err());
    assert!(AlphabeticKey::try_from("9").is_err());
    assert!(AlphabeticKey::try_from("%").is_err());
  }

  #[test]
  fn test_as_str_and_display() {
    let key = AlphabeticKey::F;
    assert_eq!(key.as_str(), "f");
    assert_eq!(key.to_string(), "f");
  }
}
