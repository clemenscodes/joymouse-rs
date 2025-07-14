mod axis;
mod direction;
mod error;
mod event;
mod keys;
mod motion;
mod polarity;
mod state;
mod vector;

pub use axis::*;
pub use direction::*;
pub use error::*;
pub use event::*;
pub use keys::*;
pub use motion::*;
pub use polarity::*;
pub use state::*;
pub use vector::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JoyStick {
  Left,
  Right,
}
