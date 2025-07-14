mod error;

pub use error::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JoyStickAxis {
  X,
  Y,
}
