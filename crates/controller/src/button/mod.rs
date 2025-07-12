mod control;
mod error;
mod event;

use crate::settings::KEYBOARD_BUTTON_MAP;
use evdev::KeyCode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

impl TryFrom<ControllerButton> for KeyCode {
  type Error = ButtonError;

  fn try_from(value: ControllerButton) -> Result<Self, Self::Error> {
    let code = match value {
      ControllerButton::South => KeyCode::BTN_SOUTH,
      ControllerButton::East => KeyCode::BTN_EAST,
      ControllerButton::North => KeyCode::BTN_WEST,
      ControllerButton::West => KeyCode::BTN_NORTH,
      ControllerButton::Up => KeyCode::BTN_DPAD_UP,
      ControllerButton::Down => KeyCode::BTN_DPAD_DOWN,
      ControllerButton::Left => KeyCode::BTN_DPAD_LEFT,
      ControllerButton::Right => KeyCode::BTN_DPAD_RIGHT,
      ControllerButton::L1 => KeyCode::BTN_TL,
      ControllerButton::R1 => KeyCode::BTN_TR,
      ControllerButton::L2 => KeyCode::BTN_TL2,
      ControllerButton::R2 => KeyCode::BTN_TR2,
      ControllerButton::L3 => KeyCode::BTN_THUMBL,
      ControllerButton::R3 => KeyCode::BTN_THUMBR,
      ControllerButton::Start => KeyCode::BTN_START,
      ControllerButton::Select => KeyCode::BTN_SELECT,
      _ => return Err(ButtonError::InvalidButton(value)),
    };
    Ok(code)
  }
}

impl TryFrom<KeyCode> for ControllerButton {
  type Error = ButtonError;

  fn try_from(value: KeyCode) -> Result<Self, Self::Error> {
    if let Some(button) = KEYBOARD_BUTTON_MAP.get(&value) {
      Ok(*button)
    } else {
      Err(ButtonError::UnsupportedKeyCode(value))
    }
  }
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

pub use error::ButtonError;
pub use event::ControllerButtonEvent;
