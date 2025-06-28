mod event;
mod state;

use crate::controller::Controller;

use std::{collections::HashMap, sync::LazyLock};

use evdev::{InputEvent, KeyCode};

pub static CONTROLLER_KEY_MAP: LazyLock<HashMap<KeyCode, ControllerButton>> = LazyLock::new(|| {
  let mut map = HashMap::new();

  map.insert(KeyCode::KEY_SPACE, ControllerButton::South);
  map.insert(KeyCode::KEY_LEFTSHIFT, ControllerButton::East);
  map.insert(KeyCode::KEY_F, ControllerButton::North);
  map.insert(KeyCode::KEY_C, ControllerButton::West);
  map.insert(KeyCode::KEY_UP, ControllerButton::Up);
  map.insert(KeyCode::KEY_LEFT, ControllerButton::Left);
  map.insert(KeyCode::KEY_DOWN, ControllerButton::Down);
  map.insert(KeyCode::KEY_RIGHT, ControllerButton::Right);
  map.insert(KeyCode::BTN_LEFT, ControllerButton::R1);
  map.insert(KeyCode::BTN_RIGHT, ControllerButton::L1);
  map.insert(KeyCode::KEY_Q, ControllerButton::L2);
  map.insert(KeyCode::KEY_X, ControllerButton::R2);
  map.insert(KeyCode::KEY_LEFTALT, ControllerButton::L3);
  map.insert(KeyCode::KEY_V, ControllerButton::R3);
  map.insert(KeyCode::KEY_TAB, ControllerButton::Select);
  map.insert(KeyCode::KEY_ENTER, ControllerButton::Start);
  map.insert(KeyCode::KEY_W, ControllerButton::Forward);
  map.insert(KeyCode::KEY_A, ControllerButton::Port);
  map.insert(KeyCode::KEY_S, ControllerButton::Backward);
  map.insert(KeyCode::KEY_D, ControllerButton::Starboard);

  for button in ControllerButton::all() {
    assert!(map.values().any(|b| b == button), "Missing mapping for ControllerButton::{:?}", button);
  }

  map
});

#[rustfmt::skip]
pub static KEYBOARD_BUTTON_MAP: LazyLock<HashMap<ControllerButton, KeyCode>> = LazyLock::new(|| 
  CONTROLLER_KEY_MAP.iter().map(|(k, v)| (*v, *k)).collect()
);

#[derive(Debug)]
pub enum ButtonError {
  UnsupportedKeyCode(KeyCode),
  InvalidState(i32),
  InvalidButton(ControllerButton),
}

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
  pub fn handle_button_event(&mut self, event: ControllerButtonEvent) {
    let virtual_event = InputEvent::from(event);
    let events = vec![virtual_event];
    self.virtual_device.emit(&events).unwrap();
  }
}

pub use event::ControllerButtonEvent;
