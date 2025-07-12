use crate::settings::SETTINGS;

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

impl Motion {
  pub fn resolve(average: f64) -> Self {
    match average {
      speed if speed >= SETTINGS.motion_flick_threshold() => Self::Flick,
      speed if speed >= SETTINGS.motion_macro_threshold() => Self::Macro,
      _ => Self::Micro,
    }
  }

  pub fn compare(average: f64, before: Self, after: Self) -> Self {
    match (before, after) {
      (Motion::Macro, Motion::Micro) if average > SETTINGS.motion_macro_sticky_lower() => {
        Motion::Macro
      }
      (Motion::Micro, Motion::Macro) if average < SETTINGS.motion_micro_sticky_upper() => {
        Motion::Micro
      }
      (_, m) => m,
    }
  }
}
