mod error;
mod event;
mod state;

use crate::controller::{Controller, settings::CONTROLLER_KEY_MAP};

use evdev::{InputEvent, KeyCode};

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

impl TryFrom<KeyCode> for ControllerButton {
  type Error = ButtonError;

  fn try_from(value: KeyCode) -> Result<Self, Self::Error> {
    if let Some(button) = CONTROLLER_KEY_MAP.get(&value) {
      return Ok(*button);
    }

    Err(ButtonError::UnsupportedKeyCode(value))
  }
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

impl Controller {
  pub fn handle_button_event(&mut self, event: ControllerButtonEvent, original: InputEvent) {
    let virtual_event = InputEvent::from(event);
    let events = vec![virtual_event];
    self.virtual_device.emit(&events).unwrap();
    if let evdev::EventSummary::Key(_, key_code, _) = original.destructure() {
      match key_code {
        KeyCode::BTN_LEFT => self.mouse_mut().emit(original), // TODO: only when not fullscreened?
        KeyCode::BTN_RIGHT => self.mouse_mut().emit(original),
        _ => (),
      };
    }
  }
}

pub use error::ButtonError;
pub use event::ControllerButtonEvent;
