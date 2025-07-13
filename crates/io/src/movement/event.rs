use crate::Movement;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct MovementEvent {
  movement: Movement,
}
