mod error;

pub use error::ModifierKeyError;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModifierKey {
  Super,
  Escape,
  Caps,

  Ctrl,
  LeftCtrl,
  RightCtrl,

  Shift,
  LeftShift,
  RightShift,

  Alt,
  LeftAlt,
  RightAlt,
}

impl ModifierKey {
  pub fn as_str(&self) -> &'static str {
    match self {
      ModifierKey::Super => "super",
      ModifierKey::Escape => "escape",
      ModifierKey::Caps => "caps",
      ModifierKey::Ctrl => "ctrl",
      ModifierKey::LeftCtrl => "left_ctrl",
      ModifierKey::RightCtrl => "right_ctrl",
      ModifierKey::Shift => "shift",
      ModifierKey::LeftShift => "left_shift",
      ModifierKey::RightShift => "right_shift",
      ModifierKey::Alt => "alt",
      ModifierKey::LeftAlt => "left_alt",
      ModifierKey::RightAlt => "right_alt",
    }
  }

  pub fn normalize(&self) -> Self {
    use ModifierKey::*;
    match self {
      Ctrl | LeftCtrl | RightCtrl => Ctrl,
      Shift | LeftShift | RightShift => Shift,
      Alt | LeftAlt | RightAlt => Alt,
      _ => *self,
    }
  }
}

impl TryFrom<&str> for ModifierKey {
  type Error = ModifierKeyError;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    match value.to_ascii_lowercase().as_str() {
      "super" => Ok(Self::Super),
      "escape" => Ok(Self::Escape),
      "caps" => Ok(Self::Caps),

      "ctrl" => Ok(Self::Ctrl),
      "left_ctrl" => Ok(Self::LeftCtrl),
      "right_ctrl" => Ok(Self::RightCtrl),

      "shift" => Ok(Self::Shift),
      "left_shift" => Ok(Self::LeftShift),
      "right_shift" => Ok(Self::RightShift),

      "alt" => Ok(Self::Alt),
      "left_alt" => Ok(Self::LeftAlt),
      "right_alt" => Ok(Self::RightAlt),

      _ => Err(ModifierKeyError::InvalidKey(value.to_owned())),
    }
  }
}

#[cfg(not(windows))]
impl TryFrom<evdev::KeyCode> for ModifierKey {
  type Error = ModifierKeyError;

  fn try_from(code: evdev::KeyCode) -> Result<Self, Self::Error> {
    Ok(match code {
      evdev::KeyCode::KEY_LEFTCTRL => Self::LeftCtrl,
      evdev::KeyCode::KEY_RIGHTCTRL => Self::RightCtrl,
      evdev::KeyCode::KEY_LEFTSHIFT => Self::LeftShift,
      evdev::KeyCode::KEY_RIGHTSHIFT => Self::RightShift,
      evdev::KeyCode::KEY_LEFTALT => Self::LeftAlt,
      evdev::KeyCode::KEY_RIGHTALT => Self::RightAlt,
      evdev::KeyCode::KEY_CAPSLOCK => Self::Caps,
      evdev::KeyCode::KEY_ESC => Self::Escape,
      evdev::KeyCode::KEY_LEFTMETA | evdev::KeyCode::KEY_RIGHTMETA => Self::Super,
      _ => return Err(ModifierKeyError::InvalidCode(code.code())),
    })
  }
}

impl std::fmt::Display for ModifierKey {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.as_str())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_try_from_valid_inputs() {
    assert_eq!(ModifierKey::try_from("ctrl").unwrap(), ModifierKey::Ctrl);
    assert_eq!(ModifierKey::try_from("LEFT_CTRL").unwrap(), ModifierKey::LeftCtrl);
    assert_eq!(ModifierKey::try_from("Right_Shift").unwrap(), ModifierKey::RightShift);
    assert_eq!(ModifierKey::try_from("alt").unwrap(), ModifierKey::Alt);
    assert_eq!(ModifierKey::try_from("escape").unwrap(), ModifierKey::Escape);
    assert_eq!(ModifierKey::try_from("SUPER").unwrap(), ModifierKey::Super);
  }

  #[test]
  fn test_try_from_invalid_input() {
    let err = ModifierKey::try_from("funky").unwrap_err();
    assert!(matches!(err, ModifierKeyError::InvalidKey(s) if s == "funky"));
  }

  #[test]
  fn test_normalize_ctrl_variants() {
    assert_eq!(ModifierKey::Ctrl.normalize(), ModifierKey::Ctrl);
    assert_eq!(ModifierKey::LeftCtrl.normalize(), ModifierKey::Ctrl);
    assert_eq!(ModifierKey::RightCtrl.normalize(), ModifierKey::Ctrl);
  }

  #[test]
  fn test_normalize_shift_variants() {
    assert_eq!(ModifierKey::Shift.normalize(), ModifierKey::Shift);
    assert_eq!(ModifierKey::LeftShift.normalize(), ModifierKey::Shift);
    assert_eq!(ModifierKey::RightShift.normalize(), ModifierKey::Shift);
  }

  #[test]
  fn test_normalize_alt_variants() {
    assert_eq!(ModifierKey::Alt.normalize(), ModifierKey::Alt);
    assert_eq!(ModifierKey::LeftAlt.normalize(), ModifierKey::Alt);
    assert_eq!(ModifierKey::RightAlt.normalize(), ModifierKey::Alt);
  }

  #[test]
  fn test_normalize_noop_for_other_keys() {
    assert_eq!(ModifierKey::Super.normalize(), ModifierKey::Super);
    assert_eq!(ModifierKey::Escape.normalize(), ModifierKey::Escape);
    assert_eq!(ModifierKey::Caps.normalize(), ModifierKey::Caps);
  }

  #[test]
  fn test_as_str_and_display() {
    let key = ModifierKey::LeftAlt;
    assert_eq!(key.as_str(), "left_alt");
    assert_eq!(key.to_string(), "left_alt");
  }

  #[cfg(not(windows))]
  mod keycode_tests {
    use super::*;
    use evdev::KeyCode;

    #[test]
    fn test_keycode_to_modifier_key() {
      assert_eq!(ModifierKey::try_from(KeyCode::KEY_LEFTCTRL).unwrap(), ModifierKey::LeftCtrl);
      assert_eq!(ModifierKey::try_from(KeyCode::KEY_RIGHTCTRL).unwrap(), ModifierKey::RightCtrl);
      assert_eq!(ModifierKey::try_from(KeyCode::KEY_LEFTSHIFT).unwrap(), ModifierKey::LeftShift);
      assert_eq!(ModifierKey::try_from(KeyCode::KEY_RIGHTSHIFT).unwrap(), ModifierKey::RightShift);
      assert_eq!(ModifierKey::try_from(KeyCode::KEY_LEFTALT).unwrap(), ModifierKey::LeftAlt);
      assert_eq!(ModifierKey::try_from(KeyCode::KEY_RIGHTALT).unwrap(), ModifierKey::RightAlt);
      assert_eq!(ModifierKey::try_from(KeyCode::KEY_ESC).unwrap(), ModifierKey::Escape);
      assert_eq!(ModifierKey::try_from(KeyCode::KEY_CAPSLOCK).unwrap(), ModifierKey::Caps);
      assert_eq!(ModifierKey::try_from(KeyCode::KEY_LEFTMETA).unwrap(), ModifierKey::Super);
      assert_eq!(ModifierKey::try_from(KeyCode::KEY_RIGHTMETA).unwrap(), ModifierKey::Super);
    }

    #[test]
    fn test_invalid_keycode_to_modifier_key() {
      let err = ModifierKey::try_from(KeyCode::KEY_A).unwrap_err();
      assert!(matches!(err, ModifierKeyError::InvalidCode(code) if code == KeyCode::KEY_A.code()));
    }
  }
}
