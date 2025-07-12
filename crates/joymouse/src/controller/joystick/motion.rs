const MOTION_FLICK_THRESHOLD: f64 = 0.5;
const MOTION_MACRO_THRESHOLD: f64 = 0.025;
const MOTION_MACRO_STICKY_LOWER: f64 = 0.01;
const MOTION_MICRO_STICKY_UPPER: f64 = 0.03;

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
      speed if speed >= MOTION_FLICK_THRESHOLD => Self::Flick,
      speed if speed >= MOTION_MACRO_THRESHOLD => Self::Macro,
      _ => Self::Micro,
    }
  }

  pub fn compare(average: f64, before: Self, after: Self) -> Self {
    match (before, after) {
      (Motion::Macro, Motion::Micro) if average > MOTION_MACRO_STICKY_LOWER => Motion::Macro,
      (Motion::Micro, Motion::Macro) if average < MOTION_MICRO_STICKY_UPPER => Motion::Micro,
      (_, m) => m,
    }
  }
}
