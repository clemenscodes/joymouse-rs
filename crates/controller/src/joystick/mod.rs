mod axis;
mod direction;
mod motion;
mod polarity;
mod state;
mod vector;

pub use axis::*;
pub use direction::*;
pub use motion::*;
pub use polarity::*;
pub use state::*;
pub use vector::*;

#[derive(Debug, PartialEq, Eq)]
pub enum JoyStick {
  Left,
  Right,
}
