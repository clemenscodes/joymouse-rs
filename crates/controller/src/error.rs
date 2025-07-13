#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ControllerError {}

impl std::fmt::Display for ControllerError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "ControllerError")
  }
}

impl std::error::Error for ControllerError {}
