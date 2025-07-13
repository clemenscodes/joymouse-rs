use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SystemKey {
  Enter,
  Tab,
  Space,
  Backspace,
}

impl SystemKey {
  pub fn as_str(&self) -> &'static str {
    match self {
      SystemKey::Enter => "enter",
      SystemKey::Tab => "tab",
      SystemKey::Space => "space",
      SystemKey::Backspace => "backspace",
    }
  }
}

impl TryFrom<&str> for SystemKey {
  type Error = SystemKeyError;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    match value.to_ascii_lowercase().as_str() {
      "enter" => Ok(Self::Enter),
      "tab" => Ok(Self::Tab),
      "space" => Ok(Self::Space),
      "backspace" => Ok(Self::Backspace),
      _ => Err(SystemKeyError::InvalidKey(value.to_owned())),
    }
  }
}

impl std::fmt::Display for SystemKey {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.as_str())
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SystemKeyError {
  InvalidKey(String),
}

impl std::fmt::Display for SystemKeyError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "invalid system key: '{}'",
      match self {
        Self::InvalidKey(s) => s,
      }
    )
  }
}

impl std::error::Error for SystemKeyError {}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_valid_system_keys() {
    assert_eq!(SystemKey::try_from("enter").unwrap(), SystemKey::Enter);
    assert_eq!(SystemKey::try_from("TAB").unwrap(), SystemKey::Tab);
    assert_eq!(SystemKey::try_from("Space").unwrap(), SystemKey::Space);
    assert_eq!(SystemKey::try_from("backspace").unwrap(), SystemKey::Backspace);
  }

  #[test]
  fn test_invalid_system_key() {
    let err = SystemKey::try_from("delete").unwrap_err();
    assert!(matches!(err, SystemKeyError::InvalidKey(ref s) if s == "delete"));
    assert_eq!(err.to_string(), "invalid system key: 'delete'");
  }

  #[test]
  fn test_as_str_and_display() {
    assert_eq!(SystemKey::Enter.as_str(), "enter");
    assert_eq!(SystemKey::Enter.to_string(), "enter");

    assert_eq!(SystemKey::Space.as_str(), "space");
    assert_eq!(SystemKey::Space.to_string(), "space");
  }
}
