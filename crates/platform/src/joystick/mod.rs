mod axis;
mod control;
mod event;
mod keys;
mod polarity;
mod state;

use controller::JoyStickError;

use evdev::{KeyCode, KeyEvent, RelativeAxisCode, RelativeAxisEvent};

#[derive(Debug, PartialEq, Eq)]
pub enum JoyStick {
  Left,
  Right,
}

impl TryFrom<RelativeAxisCode> for JoyStick {
  type Error = JoyStickError;

  fn try_from(value: RelativeAxisCode) -> Result<Self, Self::Error> {
    let joystick = match value {
      RelativeAxisCode::REL_X => Self::Right,
      RelativeAxisCode::REL_Y => Self::Right,
      other => return Err(JoyStickError::UnsupportedCode(other.0)),
    };
    Ok(joystick)
  }
}

impl TryFrom<RelativeAxisEvent> for JoyStick {
  type Error = JoyStickError;

  fn try_from(value: RelativeAxisEvent) -> Result<Self, Self::Error> {
    Self::try_from(value.code())
  }
}

impl TryFrom<KeyCode> for JoyStick {
  type Error = JoyStickError;

  fn try_from(value: KeyCode) -> Result<Self, Self::Error> {
    if !JOYSTICK_KEYS.code_is_joystick_key(value) {
      return Err(JoyStickError::UnsupportedCode(value.0));
    }
    Ok(JoyStick::Left)
  }
}

impl TryFrom<KeyEvent> for JoyStick {
  type Error = JoyStickError;

  fn try_from(value: KeyEvent) -> Result<Self, Self::Error> {
    Self::try_from(value.code())
  }
}

pub use event::ControllerJoyStickEvent;
pub use keys::JOYSTICK_KEYS;
pub use state::JoyStickState;
