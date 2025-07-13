use super::{Movement, Polarity};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct MovementEvent {
  kind: Movement,
  polarity: Polarity,
}
