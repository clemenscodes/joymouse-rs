mod axis;
mod event;
mod polarity;

pub use axis::{Axis, AxisError};
pub use event::MovementEvent;
pub use polarity::{Polarity, PolarityError};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Movement {
  axis: Axis,
  polarity: Polarity,
}
