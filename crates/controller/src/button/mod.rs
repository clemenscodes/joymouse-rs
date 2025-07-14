mod error;
mod event;
mod state;

pub use error::*;
pub use event::*;
pub use state::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ControllerButton {
  South,
  East,
  North,
  West,
  Up,
  Down,
  Left,
  Right,
  Forward,
  Backward,
  Starboard,
  Port,
  L1,
  R1,
  L2,
  R2,
  L3,
  R3,
  Start,
  Select,
}

#[rustfmt::skip]
impl ControllerButton {
  pub fn all() -> &'static [Self] {
    use ControllerButton::*;
    &[
      South, East, North, West, 
      Up, Down, Left, Right, 
      Forward, Backward, Starboard, Port, 
      L1, R1, 
      L2, R2, 
      L3, R3,
      Start, Select,
    ]
  }
}

impl std::fmt::Display for ControllerButton {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        ControllerButton::South => "south",
        ControllerButton::East => "east",
        ControllerButton::North => "north",
        ControllerButton::West => "west",
        ControllerButton::Up => "up",
        ControllerButton::Down => "down",
        ControllerButton::Left => "left",
        ControllerButton::Right => "right",
        ControllerButton::Forward => "forward",
        ControllerButton::Backward => "backward",
        ControllerButton::Starboard => "starboard",
        ControllerButton::Port => "port",
        ControllerButton::L1 => "l1",
        ControllerButton::R1 => "r1",
        ControllerButton::L2 => "l2",
        ControllerButton::R2 => "r2",
        ControllerButton::L3 => "l3",
        ControllerButton::R3 => "r3",
        ControllerButton::Start => "start",
        ControllerButton::Select => "select",
      }
    )
  }
}

impl std::str::FromStr for ControllerButton {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.to_lowercase().as_str() {
      "south" => Ok(ControllerButton::South),
      "east" => Ok(ControllerButton::East),
      "north" => Ok(ControllerButton::North),
      "west" => Ok(ControllerButton::West),
      "up" => Ok(ControllerButton::Up),
      "down" => Ok(ControllerButton::Down),
      "left" => Ok(ControllerButton::Left),
      "right" => Ok(ControllerButton::Right),
      "forward" => Ok(ControllerButton::Forward),
      "backward" => Ok(ControllerButton::Backward),
      "starboard" => Ok(ControllerButton::Starboard),
      "port" => Ok(ControllerButton::Port),
      "l1" => Ok(ControllerButton::L1),
      "r1" => Ok(ControllerButton::R1),
      "l2" => Ok(ControllerButton::L2),
      "r2" => Ok(ControllerButton::R2),
      "l3" => Ok(ControllerButton::L3),
      "r3" => Ok(ControllerButton::R3),
      "start" => Ok(ControllerButton::Start),
      "select" => Ok(ControllerButton::Select),
      _ => Err(format!("Invalid ControllerButton: {s}")),
    }
  }
}
