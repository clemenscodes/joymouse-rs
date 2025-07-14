mod event;

pub use event::MovementEvent;

use controller::{Axis, Polarity};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Movement {
  axis: Axis,
  polarity: Polarity,
}
