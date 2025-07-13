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

impl std::fmt::Display for ModifierKey {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.as_str())
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModifierKeyError {
  InvalidKey(String),
}

impl std::fmt::Display for ModifierKeyError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::InvalidKey(s) => write!(f, "invalid modifier key: '{}'", s),
    }
  }
}

impl std::error::Error for ModifierKeyError {}

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
}
