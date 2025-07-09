use crate::controller::settings::SETTINGS;

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum Motion {
  Idle,
  Micro,
  Macro,
  Flick,
}

impl Default for Motion {
  fn default() -> Self {
    Self::Idle
  }
}

impl From<Motion> for std::time::Duration {
  fn from(value: Motion) -> Self {
    match value {
      Motion::Flick => SETTINGS.tickrate(),
      Motion::Micro => SETTINGS.tickrate() * 10,
      Motion::Macro => SETTINGS.mouse_idle_timeout(),
      Motion::Idle => SETTINGS.mouse_idle_timeout(),
    }
  }
}
