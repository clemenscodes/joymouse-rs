mod error;

pub use error::AlphabeticKeyError;

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

#[cfg(not(windows))]
impl TryFrom<evdev::KeyCode> for AlphabeticKey {
  type Error = AlphabeticKeyError;

  fn try_from(code: evdev::KeyCode) -> Result<Self, Self::Error> {
    Ok(match code {
      evdev::KeyCode::KEY_A => Self::A,
      evdev::KeyCode::KEY_B => Self::B,
      evdev::KeyCode::KEY_C => Self::C,
      evdev::KeyCode::KEY_D => Self::D,
      evdev::KeyCode::KEY_E => Self::E,
      evdev::KeyCode::KEY_F => Self::F,
      evdev::KeyCode::KEY_G => Self::G,
      evdev::KeyCode::KEY_H => Self::H,
      evdev::KeyCode::KEY_I => Self::I,
      evdev::KeyCode::KEY_J => Self::J,
      evdev::KeyCode::KEY_K => Self::K,
      evdev::KeyCode::KEY_L => Self::L,
      evdev::KeyCode::KEY_M => Self::M,
      evdev::KeyCode::KEY_N => Self::N,
      evdev::KeyCode::KEY_O => Self::O,
      evdev::KeyCode::KEY_P => Self::P,
      evdev::KeyCode::KEY_Q => Self::Q,
      evdev::KeyCode::KEY_R => Self::R,
      evdev::KeyCode::KEY_S => Self::S,
      evdev::KeyCode::KEY_T => Self::T,
      evdev::KeyCode::KEY_U => Self::U,
      evdev::KeyCode::KEY_V => Self::V,
      evdev::KeyCode::KEY_W => Self::W,
      evdev::KeyCode::KEY_X => Self::X,
      evdev::KeyCode::KEY_Y => Self::Y,
      evdev::KeyCode::KEY_Z => Self::Z,
      _ => return Err(AlphabeticKeyError::InvalidCode(code.code())),
    })
  }
}

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

  #[cfg(not(windows))]
  mod keycode_tests {
    use super::*;
    use evdev::KeyCode;

    #[test]
    fn test_valid_keycodes() {
      assert_eq!(AlphabeticKey::try_from(KeyCode::KEY_A).unwrap(), AlphabeticKey::A);
      assert_eq!(AlphabeticKey::try_from(KeyCode::KEY_Z).unwrap(), AlphabeticKey::Z);
    }

    #[test]
    fn test_invalid_keycode() {
      let result = AlphabeticKey::try_from(KeyCode::KEY_1);
      assert!(
        matches!(result, Err(AlphabeticKeyError::InvalidCode(code)) if code == KeyCode::KEY_1.code())
      );
    }
  }
}
