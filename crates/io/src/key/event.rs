use crate::{key::state::KeyState, Key};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct KeyEvent {
  kind: Key,
  state: KeyState,
  value: i32,
}
