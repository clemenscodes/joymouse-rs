mod control;
mod error;
mod event;

pub use event::*;

use crate::keys::KEYBOARD_BUTTON_MAP;
use controller::{ButtonError, ControllerButton};
use evdev::KeyCode;

pub fn try_key_code_from_controller_button(
  button: ControllerButton,
) -> Result<KeyCode, ButtonError> {
  let code = match button {
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
    _ => return Err(ButtonError::InvalidButton(button)),
  };
  Ok(code)
}

pub fn try_controller_button_from_keycode(code: KeyCode) -> Result<ControllerButton, ButtonError> {
  if let Some(button) = KEYBOARD_BUTTON_MAP.get(&code) {
    Ok(*button)
  } else {
    Err(ButtonError::UnsupportedKeyCode(code.code()))
  }
}
