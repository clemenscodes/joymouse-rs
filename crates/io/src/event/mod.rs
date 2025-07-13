use crate::{KeyEvent, MovementEvent};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum InputEvent {
  Key(KeyEvent),
  Movement(MovementEvent),
}
