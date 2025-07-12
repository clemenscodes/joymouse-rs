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
