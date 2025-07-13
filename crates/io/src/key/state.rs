#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum KeyState {
  Pressed,
  Released,
  Held,
  Moved,
}
