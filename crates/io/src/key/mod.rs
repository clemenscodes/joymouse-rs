mod alphabetic;
mod arrow;
mod error;
mod function;
mod modifier;
mod mouse;
mod numeric;
mod state;
mod system;

pub use alphabetic::{AlphabeticKey, AlphabeticKeyError};
pub use arrow::{ArrowKey, ArrowKeyError};
pub use error::KeyError;
pub use function::{FunctionKey, FunctionKeyError};
pub use modifier::{ModifierKey, ModifierKeyError};
pub use mouse::{MouseKey, MouseKeyError};
pub use numeric::{NumericKey, NumericKeyError};
pub use state::KeyState;
pub use system::{SystemKey, SystemKeyError};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Key {
  Alphabetic(AlphabeticKey),
  Numeric(NumericKey),
  Function(FunctionKey),
  Arrow(ArrowKey),
  Modifier(ModifierKey),
  System(SystemKey),
  Mouse(MouseKey),
}

impl Key {
  pub fn normalize(&self) -> Self {
    match self {
      Key::Modifier(modifier) => Self::from(modifier.normalize()),
      _ => *self,
    }
  }

  pub fn as_str(&self) -> &'static str {
    match self {
      Key::Alphabetic(k) => k.as_str(),
      Key::Numeric(k) => k.as_str(),
      Key::Function(k) => k.as_str(),
      Key::Arrow(k) => k.as_str(),
      Key::Modifier(k) => k.as_str(),
      Key::System(k) => k.as_str(),
      Key::Mouse(k) => k.as_str(),
    }
  }
}

impl From<AlphabeticKey> for Key {
  fn from(v: AlphabeticKey) -> Self {
    Self::Alphabetic(v)
  }
}

impl From<NumericKey> for Key {
  fn from(v: NumericKey) -> Self {
    Self::Numeric(v)
  }
}

impl From<FunctionKey> for Key {
  fn from(v: FunctionKey) -> Self {
    Self::Function(v)
  }
}

impl From<ArrowKey> for Key {
  fn from(v: ArrowKey) -> Self {
    Self::Arrow(v)
  }
}

impl From<ModifierKey> for Key {
  fn from(v: ModifierKey) -> Self {
    Self::Modifier(v)
  }
}

impl From<SystemKey> for Key {
  fn from(v: SystemKey) -> Self {
    Self::System(v)
  }
}

impl From<MouseKey> for Key {
  fn from(v: MouseKey) -> Self {
    Self::Mouse(v)
  }
}

impl TryFrom<&str> for Key {
  type Error = KeyError;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    let key = AlphabeticKey::try_from(value)
      .map(Self::Alphabetic)
      .or_else(|_| NumericKey::try_from(value).map(Self::Numeric))
      .or_else(|_| FunctionKey::try_from(value).map(Self::Function))
      .or_else(|_| ArrowKey::try_from(value).map(Self::Arrow))
      .or_else(|_| ModifierKey::try_from(value).map(Self::Modifier))
      .or_else(|_| SystemKey::try_from(value).map(Self::System))
      .or_else(|_| MouseKey::try_from(value).map(Self::Mouse))?;
    Ok(key)
  }
}

#[cfg(not(windows))]
impl TryFrom<evdev::KeyCode> for Key {
  type Error = KeyError;

  fn try_from(code: evdev::KeyCode) -> Result<Self, Self::Error> {
    let key = AlphabeticKey::try_from(code)
      .map(Self::Alphabetic)
      .or_else(|_| NumericKey::try_from(code).map(Self::Numeric))
      .or_else(|_| FunctionKey::try_from(code).map(Self::Function))
      .or_else(|_| ArrowKey::try_from(code).map(Self::Arrow))
      .or_else(|_| ModifierKey::try_from(code).map(Self::Modifier))
      .or_else(|_| SystemKey::try_from(code).map(Self::System))
      .or_else(|_| MouseKey::try_from(code).map(Self::Mouse))?;
    Ok(key)
  }
}

impl std::fmt::Display for Key {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.as_str())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_try_from_str_valid() {
    assert_eq!(Key::try_from("a").unwrap(), Key::Alphabetic(AlphabeticKey::A));
    assert_eq!(Key::try_from("1").unwrap(), Key::Numeric(NumericKey::Num1));
    assert_eq!(Key::try_from("f2").unwrap(), Key::Function(FunctionKey::F2));
    assert_eq!(Key::try_from("left").unwrap(), Key::Arrow(ArrowKey::Left));
    assert_eq!(Key::try_from("ctrl").unwrap(), Key::Modifier(ModifierKey::Ctrl));
    assert_eq!(Key::try_from("space").unwrap(), Key::System(SystemKey::Space));
    assert_eq!(Key::try_from("mouse_left").unwrap(), Key::Mouse(MouseKey::Left));
  }

  #[test]
  fn test_try_from_str_invalid() {
    let err = Key::try_from("foobar").unwrap_err();
    assert!(matches!(
      err,
      KeyError::Alphabetic(_)
        | KeyError::Numeric(_)
        | KeyError::Function(_)
        | KeyError::Arrow(_)
        | KeyError::Modifier(_)
        | KeyError::System(_)
        | KeyError::Mouse(_)
    ));
  }

  #[test]
  fn test_key_as_str() {
    let key = Key::Alphabetic(AlphabeticKey::Z);
    assert_eq!(key.as_str(), "z");
    assert_eq!(key.to_string(), "z");

    let key = Key::Function(FunctionKey::F5);
    assert_eq!(key.as_str(), "f5");
    assert_eq!(key.to_string(), "f5");

    let key = Key::Mouse(MouseKey::Right);
    assert_eq!(key.as_str(), "mouse_right");
    assert_eq!(key.to_string(), "mouse_right");
  }

  #[test]
  fn test_key_normalization() {
    let raw = Key::Modifier(ModifierKey::RightCtrl);
    assert_eq!(raw.normalize(), Key::Modifier(ModifierKey::Ctrl));

    let raw = Key::Modifier(ModifierKey::Alt);
    assert_eq!(raw.normalize(), Key::Modifier(ModifierKey::Alt));

    let raw = Key::Arrow(ArrowKey::Up);
    assert_eq!(raw.normalize(), raw);
  }

  #[test]
  fn test_from_variants() {
    let alpha: Key = AlphabeticKey::H.into();
    assert_eq!(alpha, Key::Alphabetic(AlphabeticKey::H));

    let arrow: Key = ArrowKey::Down.into();
    assert_eq!(arrow, Key::Arrow(ArrowKey::Down));

    let modkey: Key = ModifierKey::Shift.into();
    assert_eq!(modkey, Key::Modifier(ModifierKey::Shift));
  }

  #[cfg(not(windows))]
  mod keycode_tests {
    use super::*;
    use evdev::KeyCode;

    #[test]
    fn test_keycode_to_key_conversion() {
      assert_eq!(Key::try_from(KeyCode::KEY_A).unwrap(), Key::Alphabetic(AlphabeticKey::A));
      assert_eq!(Key::try_from(KeyCode::KEY_1).unwrap(), Key::Numeric(NumericKey::Num1));
      assert_eq!(Key::try_from(KeyCode::KEY_F5).unwrap(), Key::Function(FunctionKey::F5));
      assert_eq!(Key::try_from(KeyCode::KEY_LEFT).unwrap(), Key::Arrow(ArrowKey::Left));
      assert_eq!(
        Key::try_from(KeyCode::KEY_LEFTCTRL).unwrap(),
        Key::Modifier(ModifierKey::LeftCtrl)
      );
      assert_eq!(Key::try_from(KeyCode::KEY_SPACE).unwrap(), Key::System(SystemKey::Space));
      assert_eq!(Key::try_from(KeyCode::BTN_LEFT).unwrap(), Key::Mouse(MouseKey::Left));
    }

    #[test]
    fn test_keycode_to_key_invalid() {
      let err = Key::try_from(KeyCode::KEY_COMPOSE).unwrap_err();
      assert!(matches!(
        err,
        KeyError::Alphabetic(_)
          | KeyError::Numeric(_)
          | KeyError::Function(_)
          | KeyError::Arrow(_)
          | KeyError::Modifier(_)
          | KeyError::System(_)
          | KeyError::Mouse(_)
      ));
    }
  }
}
