mod error;

pub use error::AxisError;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Axis {
  X,
  Y,
}
