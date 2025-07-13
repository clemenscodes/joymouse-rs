use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MouseKey {
  Left,
  Right,
  Middle,
  Side,
  Extra,
}

impl MouseKey {
  pub fn as_str(&self) -> &'static str {
    match self {
      MouseKey::Left => "mouse_left",
      MouseKey::Right => "mouse_right",
      MouseKey::Middle => "mouse_middle",
      MouseKey::Side => "mouse_side",
      MouseKey::Extra => "mouse_extra",
    }
  }
}

impl TryFrom<&str> for MouseKey {
  type Error = MouseKeyError;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    match value.to_ascii_lowercase().as_str() {
      "mouse_left" => Ok(Self::Left),
      "mouse_right" => Ok(Self::Right),
      "mouse_middle" => Ok(Self::Middle),
      "mouse_side" => Ok(Self::Side),
      "mouse_extra" => Ok(Self::Extra),
      _ => Err(MouseKeyError::InvalidKey(value.to_owned())),
    }
  }
}

#[cfg(not(windows))]
impl TryFrom<evdev::KeyCode> for MouseKey {
  type Error = MouseKeyError;

  fn try_from(code: evdev::KeyCode) -> Result<Self, Self::Error> {
    Ok(match code {
      evdev::KeyCode::BTN_LEFT => Self::Left,
      evdev::KeyCode::BTN_RIGHT => Self::Right,
      evdev::KeyCode::BTN_MIDDLE => Self::Middle,
      evdev::KeyCode::BTN_SIDE => Self::Side,
      evdev::KeyCode::BTN_EXTRA => Self::Extra,
      _ => return Err(MouseKeyError::InvalidCode(code.code())),
    })
  }
}

impl std::fmt::Display for MouseKey {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.as_str())
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MouseKeyError {
  InvalidKey(String),
  InvalidCode(u16),
}

impl std::fmt::Display for MouseKeyError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::InvalidKey(s) => write!(f, "invalid mouse key: '{}'", s),
      Self::InvalidCode(code) => write!(f, "invalid mouse key code: '{}'", code),
    }
  }
}

impl std::error::Error for MouseKeyError {}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_valid_mouse_keys() {
    assert_eq!(MouseKey::try_from("mouse_left").unwrap(), MouseKey::Left);
    assert_eq!(MouseKey::try_from("MOUSE_RIGHT").unwrap(), MouseKey::Right);
    assert_eq!(MouseKey::try_from("Mouse_Middle").unwrap(), MouseKey::Middle);
    assert_eq!(MouseKey::try_from("mouse_side").unwrap(), MouseKey::Side);
    assert_eq!(MouseKey::try_from("mouse_extra").unwrap(), MouseKey::Extra);
  }

  #[test]
  fn test_invalid_mouse_key() {
    let err = MouseKey::try_from("click").unwrap_err();
    assert!(matches!(err, MouseKeyError::InvalidKey(ref s) if s == "click"));
    assert_eq!(err.to_string(), "invalid mouse key: 'click'");
  }

  #[test]
  fn test_as_str_and_display_output() {
    assert_eq!(MouseKey::Left.as_str(), "mouse_left");
    assert_eq!(MouseKey::Left.to_string(), "mouse_left");

    assert_eq!(MouseKey::Right.as_str(), "mouse_right");
    assert_eq!(MouseKey::Right.to_string(), "mouse_right");
  }

  #[cfg(not(windows))]
  mod keycode_tests {
    use super::*;
    use evdev::KeyCode;

    #[test]
    fn test_mouse_key_from_evdev_keycode() {
      assert_eq!(MouseKey::try_from(KeyCode::BTN_LEFT).unwrap(), MouseKey::Left);
      assert_eq!(MouseKey::try_from(KeyCode::BTN_RIGHT).unwrap(), MouseKey::Right);
      assert_eq!(MouseKey::try_from(KeyCode::BTN_MIDDLE).unwrap(), MouseKey::Middle);
      assert_eq!(MouseKey::try_from(KeyCode::BTN_SIDE).unwrap(), MouseKey::Side);
      assert_eq!(MouseKey::try_from(KeyCode::BTN_EXTRA).unwrap(), MouseKey::Extra);
    }

    #[test]
    fn test_invalid_mouse_keycode() {
      let err = MouseKey::try_from(KeyCode::KEY_A).unwrap_err();
      assert!(matches!(err, MouseKeyError::InvalidCode(code) if code == KeyCode::KEY_A.code()));
    }
  }
}
