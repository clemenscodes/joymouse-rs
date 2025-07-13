use settings::SETTINGS;

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

impl From<f64> for Motion {
  fn from(avg_speed: f64) -> Self {
    if avg_speed >= SETTINGS.motion_threshold_macro_flick() {
      Motion::Flick
    } else if avg_speed >= SETTINGS.motion_threshold_micro_macro() {
      Motion::Macro
    } else {
      Motion::Micro
    }
  }
}

impl From<Motion> for std::time::Duration {
  fn from(value: Motion) -> Self {
    match value {
      Motion::Idle => SETTINGS.mouse_idle_timeout(),
      Motion::Micro => SETTINGS.mouse_idle_timeout() * 2,
      Motion::Macro => SETTINGS.mouse_idle_timeout(),
      Motion::Flick => SETTINGS.mouse_idle_timeout(),
    }
  }
}
