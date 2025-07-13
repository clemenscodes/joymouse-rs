mod absolute;
mod axis;
mod event;
mod polarity;
mod relative;

pub use absolute::AbsoluteMovement;
pub use axis::Axis;
pub use event::MovementEvent;
pub use polarity::Polarity;
pub use relative::RelativeMovement;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Movement {
  Absolute(AbsoluteMovement),
  Relative(RelativeMovement),
}

impl From<RelativeMovement> for Movement {
  fn from(v: RelativeMovement) -> Self {
    Self::Relative(v)
  }
}

impl From<AbsoluteMovement> for Movement {
  fn from(v: AbsoluteMovement) -> Self {
    Self::Absolute(v)
  }
}
