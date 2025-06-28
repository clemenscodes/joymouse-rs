use std::sync::LazyLock;

#[derive(Debug)]
pub struct ControllerSettings {
  sensitivity: i32,
}

impl ControllerSettings {
  pub fn sensitivity(&self) -> i32 {
    self.sensitivity
  }
}

impl Default for ControllerSettings {
  fn default() -> Self {
    Self {
      sensitivity: 100,
    }
  }
}

pub static SETTINGS: LazyLock<ControllerSettings> = LazyLock::new(ControllerSettings::default);
